use crate::fps::Fps;
use crate::img::Images;
use crate::loader::Loader;
use crate::messages::Messages;
use crate::panel::PanelProgram;
use crate::{font::Fonts, log, App, AppConfig, AppInput};
pub use atomic_refcell::{AtomicRef, AtomicRefMut, BorrowError, BorrowMutError};
use lazy_static::lazy_static;
use legion::serialize::Canon;
pub use legion::storage::Component;
use legion::systems::Resource;
pub use legion::systems::ResourceSet;
pub use legion::Registry;
pub use legion::*;
use serde::de::DeserializeSeed;
pub use serde::{Deserialize, Serialize};
use std::sync::Mutex;

lazy_static! {
    pub static ref REGISTRY: Mutex<Registry<String>> = Mutex::new(Registry::new());
}

pub fn register_component<C>(name: &str)
where
    for<'d> C: Component + Serialize + Deserialize<'d>,
{
    if let Ok(mut registry) = REGISTRY.lock() {
        registry.register::<C>(name.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub size: (u32, u32),
    pub real_size: (u32, u32),
    pub screen_size: (u32, u32),
    pub hidpi_factor: f32,
}

#[derive(Debug, Clone)]
pub struct Time {
    pub now: f64,
    pub delta: f64,
}

impl Time {
    pub fn new(now: f64, delta: f64) -> Self {
        Time { now, delta }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            now: 0.0,
            delta: 0.0,
        }
    }
}

pub struct Ecs {
    pub world: World,
    pub resources: Resources,
}

impl Ecs {
    pub fn new() -> Self {
        Ecs {
            world: World::default(),
            resources: Resources::default(),
        }
    }
}

pub fn init_ecs(ecs: &mut Ecs, app: &App, options: &AppConfig) {
    let resources = &mut ecs.resources;

    // FPS
    resources.insert(Fps::new(options.fps));

    // Window Sizes
    let real_window_width = (options.size.0 as f32 * app.hidpi_factor()) as u32;
    let real_window_height = (options.size.1 as f32 * app.hidpi_factor()) as u32;

    let screen_resolution = app.screen_resolution();

    let (x_offset, y_offset) = if options.fullscreen && cfg!(not(target_arch = "wasm32")) {
        let x_offset = (screen_resolution.0 - real_window_width) as i32 / 2;
        let y_offset = (screen_resolution.1 - real_window_height) as i32 / 2;
        (x_offset, y_offset)
    } else {
        (0, 0)
    };

    let window_info = WindowInfo {
        size: options.size,
        real_size: (real_window_width, real_window_height),
        screen_size: screen_resolution,
        hidpi_factor: app.hidpi_factor(),
    };

    log(&format!(
        "Screen size: {:?}, window_size: {:?}, offset {}x{}, real_window_size: {:?},  hidpi factor: {}",
        window_info.screen_size,
        window_info.size,
        x_offset,
        y_offset,
        window_info.real_size,
        window_info.hidpi_factor
    ));

    resources.insert(window_info);

    // GL + Panel Program
    let gl = uni_gl::WebGLRenderingContext::new(app.canvas());
    gl.viewport(x_offset, y_offset, real_window_width, real_window_height);
    gl.enable(uni_gl::Flag::Blend as i32);
    // gl.enable(uni_gl::Flag::DepthTest as i32);   // If using ZPos
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(uni_gl::BufferBit::Color);
    // gl.clear(uni_gl::BufferBit::Depth);  // If using ZPos
    gl.blend_equation(uni_gl::BlendEquation::FuncAdd);
    gl.blend_func(
        uni_gl::BlendMode::SrcAlpha,
        uni_gl::BlendMode::OneMinusSrcAlpha,
    );

    resources.insert(PanelProgram::new(&gl));

    resources.insert(Fonts::new(&gl));
    resources.insert(Images::new());

    resources.insert(gl);

    // App Input
    let input = if cfg!(target_arch = "wasm32") {
        AppInput::new(
            (options.size.0, options.size.1),
            // (options.console_width, options.console_height),
            (x_offset as u32, y_offset as u32),
        )
    } else {
        AppInput::new(
            (real_window_width, real_window_height),
            // (options.console_width, options.console_height),
            (x_offset as u32, y_offset as u32),
        )
    };
    resources.insert(input);

    resources.insert(Time::default());
    resources.insert(Messages::new());
    resources.insert(Loader::new());

    log("Configured ECS");
}

pub fn scoped_resource<F, R, T>(ecs: &mut Ecs, func: F) -> T
where
    F: FnOnce(&mut Ecs, &mut R) -> T,
    R: Resource,
{
    let mut resource = ecs.resources.remove::<R>().unwrap();
    let result = func(ecs, &mut resource);
    ecs.resources.insert(resource);
    return result;
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveToNewWorld;

pub fn move_entity(entity: Entity, src: &mut World, dest: &mut World) -> Entity {
    let mut registry = REGISTRY.lock().unwrap();

    // add the marker component
    let mut entry = src.entry(entity).unwrap();
    entry.add_component(MoveToNewWorld);

    registry.register::<MoveToNewWorld>("MoveToNewWorld".to_string());

    // let mut query = <(Entity, &MoveToNewWorld)>::query();
    // let src_heros: Vec<Entity> = query.iter(src).map(|(e, _)| *e).collect();

    // println!("Entities to move - {:?}", src_heros);

    let filter = component::<MoveToNewWorld>();
    let entity_serializer = Canon::default();

    let json = serde_json::to_value(&src.as_serializable(filter, &*registry, &entity_serializer))
        .expect("Failed to serialize world!");
    // println!("JSON = {:#}", json);

    // for hero in src_heros {
    // println!("- Deleting = {:?}", hero);
    // src.remove(hero); // Delete the original entity
    // }

    src.remove(entity);

    let mut query = <(Entity, &MoveToNewWorld)>::query();

    // let heros: Vec<Entity> = query.iter(dest).map(|(e, _)| *e).collect();
    // println!("Dest starting entities = {:?}", heros);

    // registries are also serde deserializers
    registry
        .as_deserialize_into_world(dest, &entity_serializer)
        .deserialize(json)
        .expect("Failed to deserialize world!");

    let new_entities: Vec<Entity> = query.iter(dest).map(|(e, _)| *e).collect();

    let entity = new_entities.into_iter().next().unwrap();

    let mut entry = dest.entry(entity).unwrap();
    entry.remove_component::<MoveToNewWorld>();

    entity
}
