use gw_app::*;
use gw_ecs::prelude::*;
use gw_ui::css::*;
use gw_ui::screens::{Choice, MultiChoice};
use gw_ui::ui::*;
use gw_util::value::{Key, Value};
use std::collections::HashMap;
use std::sync::Arc;

struct MainScreen {
    ui: UI,
    // schedule: Schedule,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let ui = page((80, 50), "DEFAULT", |body| {
            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("] Manage Inventory [")
                    .pos(40, 5)
                    .spacing(1)
                    .width(32);

                Button::new(frame, |btn| {
                    btn.id("PICKUP")
                        .text("[ Pickup Item ]")
                        .activate_key(VirtualKeyCode::Return);
                });

                Button::new(frame, |btn| {
                    btn.id("SHOW")
                        .text("[ See Inventory ]")
                        .activate_key(VirtualKeyCode::Return);
                });

                Button::new(frame, |btn| {
                    btn.id("DROP")
                        .text("[ Drop Item ]")
                        .activate_key(VirtualKeyCode::Return);
                });
            });

            Frame::new(body, |frame| {
                frame.margin(1).title("] Inventory [").pos(40, 15);

                Text::new(frame, |txt| {
                    txt.id("INVENTORY")
                        .width(30)
                        .text("Nothing.")
                        .height(20)
                        .bg("dark_blue".into());
                });
            });

            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("] Manage Floor Items [")
                    .pos(5, 5)
                    .spacing(1)
                    .size(34, 9);

                Button::new(frame, |btn| {
                    btn.id("CREATE")
                        .text("[ Create Item ]")
                        .activate_key(VirtualKeyCode::Return);
                });

                Button::new(frame, |btn| {
                    btn.id("DELETE")
                        .text("[ Delete Item ]")
                        .activate_key(VirtualKeyCode::Return);
                });
            });

            Frame::new(body, |frame| {
                frame.margin(1).title("] Items on Floor [").pos(5, 15);

                Text::new(frame, |txt| {
                    txt.id("FLOOR")
                        .width(30)
                        .text("Nothing.")
                        .height(20)
                        .bg("dark_blue".into());
                });
            });

            Text::new(body, |txt| {
                txt.id("TEXT")
                    .text("Nothing")
                    .width(80)
                    .height(3)
                    .pos(5, 45);
            });
        });

        ui.dump();

        Box::new(MainScreen {
            ui,
            // schedule: Schedule::builder().build(),
        })
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, app: &mut Ecs) {
        init_item_kinds(app);
        self.ui.update_styles();
    }

    fn input(&mut self, app: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.ui.input(app, ev) {
            println!("- input={:?}", result);
            return result;
        }
        ScreenResult::Continue
    }

    fn message(&mut self, app: &mut Ecs, id: &str, value: Option<Value>) -> ScreenResult {
        match id {
            "CREATE" => {
                let item_kinds = app.read_global::<ItemKinds>();

                let items: Vec<(String, Value)> = item_kinds
                    .iter()
                    .map(|kind| (kind.name.clone(), kind.id.clone().into()))
                    .collect();

                return ScreenResult::Push(
                    Choice::builder("CREATE_ITEM")
                        .title("Create which item?")
                        .items(items)
                        .class("blue-back")
                        // .checkbox()
                        .build(),
                );
            }
            "CREATE_ITEM" => {
                let text = self.ui.find_by_id("TEXT").unwrap();
                let kind_id = match value {
                    None => {
                        text.set_text("Nothing");
                        return ScreenResult::Continue;
                    }
                    Some(v) => v.to_string(),
                };

                text.set_text(&kind_id);

                let kind = app.read_global::<ItemKinds>().get(&kind_id).unwrap();

                if stack_floor(app.current_world_mut(), &kind, 1).is_none() {
                    create_floor(app.current_world_mut(), &kind, 1);
                }
            }

            "DELETE" => {
                let (entities, items, in_inventory) =
                    <(Entities, ReadComp<Item>, ReadComp<InInventory>)>::fetch(app.current_world());

                // let mut query = <(Entity, Read<Item>)>::query().filter(!component::<InInventory>());

                let items: Vec<(String, Key, u16)> = (&entities, &items, !&in_inventory)
                    .join()
                    .map(|(entity, item, _)| {
                        (
                            format!("{} {}", item.count, item.kind.name),
                            entity.into(),
                            item.count,
                        )
                    })
                    .collect();

                if items.len() > 0 {
                    return ScreenResult::Push(
                        MultiChoice::builder("DELETE_ITEM")
                            .title("Delete which item(s)?")
                            .count("#")
                            .items(items)
                            .class("blue-back")
                            .build(),
                    );
                } else {
                    // TODO - return ScreenResult::Push(MsgBox::builder("MSGBOX").title("Error").prompt("Nothing to delete.").build());
                    return ScreenResult::Continue;
                }
            }

            "DELETE_ITEM" => {
                if let Some(Value::Map(map)) = value {
                    for (key, val) in map {
                        let entity: Entity = key.try_into().unwrap();
                        let count: i32 = val.try_into().unwrap();

                        let world = app.current_world_mut();
                        if reduce_count(world, entity, count as u16) == 0 {
                            let _ = world.delete_entity(entity);
                            log(format!("Delete item - {:?}", entity));
                        }
                    }
                }
            }

            "PICKUP" => {
                let world = app.current_world();
                let (entities, items, in_inventory) =
                    <(Entities, ReadComp<Item>, ReadComp<InInventory>)>::fetch(world);

                let query = (&entities, &items, !&in_inventory).join();

                let items: Vec<(String, Key, u16)> = query
                    .map(|(entity, item, _)| {
                        (
                            format!("{} {}", item.count, item.kind.name),
                            entity.into(),
                            item.count,
                        )
                    })
                    .collect();

                if items.len() > 0 {
                    return ScreenResult::Push(
                        MultiChoice::builder("PICKUP_ITEM")
                            .title("Pickup which item(s)?")
                            .count("#")
                            .items(items)
                            .class("blue-back")
                            .build(),
                    );
                } else {
                    // TODO - return ScreenResult::Push(MsgBox::builder("MSGBOX").title("Error").prompt("Nothing to pickup.").build());
                    return ScreenResult::Continue;
                }
            }
            "PICKUP_ITEM" => {
                if let Some(Value::Map(map)) = value {
                    for (key, val) in map {
                        let entity: Entity = key.try_into().unwrap();
                        let count: i32 = val.try_into().unwrap();

                        let world = app.current_world_mut();

                        let (kind_id, has_count) = get_info(world, entity);

                        match has_count == count as u16 {
                            true => match stack_inventory(world, &kind_id, count as u16) {
                                None => {
                                    let _ =
                                        world.write_component().insert(entity, InInventory::new());
                                }
                                Some(_) => {
                                    let _ = world.delete_entity(entity);
                                }
                            },
                            false => {
                                reduce_count(world, entity, count as u16);
                                if stack_inventory(world, &kind_id, count as u16).is_none() {
                                    create_inventory(world, &kind_id, count as u16);
                                }
                            }
                        }
                    }
                }
            }

            "SHOW" => {
                // return ScreenResult::Push(
                //     PickItem::builder("PICK")
                //         .title("Drop which item(s)?")
                //         // .items()
                //         .class("blue-back")
                //         .build(),
                // );
            }

            "DROP" => {
                let world = app.current_world();
                let (entities, items, in_inventory) =
                    <(Entities, ReadComp<Item>, ReadComp<InInventory>)>::fetch(world);

                let query = (&entities, &items, &in_inventory).join();

                let items: Vec<(String, Key, u16)> = query
                    .map(|(entity, item, _)| {
                        (
                            format!("{} {}", item.count, item.kind.id),
                            entity.try_into().unwrap(),
                            item.count,
                        )
                    })
                    .collect();

                if items.len() > 0 {
                    return ScreenResult::Push(
                        MultiChoice::builder("DROP_ITEM")
                            .title("Drop which item(s)?")
                            .count("#")
                            .items(items)
                            .class("blue-back")
                            .build(),
                    );
                } else {
                    // TODO - return ScreenResult::Push(MsgBox::builder("MSGBOX").title("Error").prompt("Nothing to drop.").build());
                    return ScreenResult::Continue;
                }
            }
            "DROP_ITEM" => {
                if let Some(Value::Map(map)) = value {
                    for (key, val) in map {
                        let entity: Entity = key.try_into().unwrap();
                        let count: i32 = val.try_into().unwrap();

                        let world = app.current_world_mut();
                        let (kind, has_count) = get_info(world, entity);

                        match has_count == count as u16 {
                            true => {
                                log("Removing whole inventory item");
                                if stack_floor(world, &kind, count as u16).is_some() {
                                    let _ = world.delete_entity(entity);
                                } else {
                                    world.write_component::<InInventory>().remove(entity);
                                }
                            }
                            false => {
                                reduce_count(world, entity, count as u16);
                                if stack_floor(world, &kind, count as u16).is_none() {
                                    create_floor(world, &kind, count as u16);
                                }
                            }
                        }
                    }
                }
            }

            _ => return ScreenResult::Continue,
        }

        // update floor items
        let world = app.current_world();
        let (entities, items, in_inventory) =
            <(Entities, ReadComp<Item>, ReadComp<InInventory>)>::fetch(world);

        let query = (&entities, &items, !&in_inventory).join();

        let ids: Vec<String> = query
            .map(|(_, item, _)| format!("{} {}", item.count, item.kind.name))
            .collect();

        let floor = self.ui.find_by_id("FLOOR").unwrap();
        match ids.is_empty() {
            true => floor.set_text("Nothing."),
            false => floor.set_text(&ids.join("\n")),
        }

        // update inventory items
        let query = (&entities, &items, &in_inventory).join();
        let ids: Vec<String> = query
            .map(|(_, item, _)| format!("{} {}", item.count, item.kind.name))
            .collect();

        let floor = self.ui.find_by_id("INVENTORY").unwrap();
        match ids.is_empty() {
            true => floor.set_text("Nothing."),
            false => floor.set_text(&ids.join("\n")),
        }

        log("Update UI");
        ScreenResult::Continue
    }

    fn update(&mut self, _ecs: &mut Ecs) -> ScreenResult {
        // self.schedule.execute(&mut app.world, &mut app.resources);

        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        self.ui.render(app);
    }

    fn teardown(&mut self, _app: &mut Ecs) {}
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Inventory Example")
        .file(
            "assets/styles.css",
            Box::new(|path: &str, data: Vec<u8>, app: &mut Ecs| {
                let r = load_stylesheet_data(path, data, app);
                if r.is_ok() {
                    STYLES.lock().unwrap().dump();
                }
                r
            }),
        )
        .register_components(|ecs| {
            ecs.register::<Item>();
            ecs.register::<InInventory>();

            ecs.create_world("MAIN");
        })
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

#[derive(Clone)]
struct ItemKind {
    id: String,
    name: String,
    stackable: bool,
}

impl ItemKind {
    fn new(id: &str, name: &str, stackable: bool) -> Self {
        ItemKind {
            id: id.to_string(),
            name: name.to_string(),
            stackable,
        }
    }
}

#[derive(Clone, Default)]
struct ItemKinds {
    kinds: HashMap<String, Arc<ItemKind>>,
}

impl ItemKinds {
    fn new() -> Self {
        ItemKinds {
            kinds: HashMap::new(),
        }
    }

    fn insert(&mut self, kind: ItemKind) {
        self.kinds.insert(kind.id.clone(), Arc::new(kind));
    }

    fn get(&self, id: &str) -> Option<Arc<ItemKind>> {
        match self.kinds.get(id) {
            None => None,
            Some(k) => Some(k.clone()),
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Arc<ItemKind>> {
        self.kinds.values()
    }
}

fn init_item_kinds(resources: &mut Ecs) {
    let mut item_kinds = ItemKinds::new();
    item_kinds.insert(ItemKind::new("TACO", "taco", true));
    item_kinds.insert(ItemKind::new("BURRITO", "burrito", true));
    item_kinds.insert(ItemKind::new("SOMBRERO", "sombrero", false));
    item_kinds.insert(ItemKind::new("CACTUS", "cactus", false));

    resources.insert_global(item_kinds);
}

#[derive(Clone, Component)]
struct Item {
    kind: Arc<ItemKind>,
    count: u16,
}

impl Item {
    fn new(kind: Arc<ItemKind>) -> Self {
        Item { kind, count: 1 }
    }
}

#[derive(Clone, Component)]
struct InInventory {}

impl InInventory {
    fn new() -> Self {
        InInventory {}
    }
}

fn stack_floor(world: &mut World, kind: &Arc<ItemKind>, count: u16) -> Option<Entity> {
    if !kind.stackable {
        return None;
    }

    let (entities, mut items, in_inventory) =
        <(Entities, WriteComp<Item>, ReadComp<InInventory>)>::fetch(world);

    match (&entities, &mut items, !&in_inventory)
        .join()
        .find(|(_, item, _)| item.kind.id == kind.id)
    {
        None => None,
        Some((entity, mut same_kind_item, _)) => {
            same_kind_item.count += count;
            log(format!("Added {} to existing floor item.", count));
            Some(entity)
        }
    }
}

fn create_floor(world: &mut World, kind: &Arc<ItemKind>, count: u16) -> Entity {
    let mut item = Item::new(kind.clone());
    item.count = count;
    let entity = world.create_entity().with(item).id();
    log(format!("Created floor entity - {:?}", entity));
    entity
}

fn stack_inventory(world: &mut World, kind: &Arc<ItemKind>, count: u16) -> Option<Entity> {
    if !kind.stackable {
        return None;
    }

    let (entities, mut items, in_inventory) =
        <(Entities, WriteComp<Item>, ReadComp<InInventory>)>::fetch(world);

    match (&entities, &mut items, !&in_inventory)
        .join()
        .find(|(_, item, _)| item.kind.id == kind.id)
    {
        None => None,
        Some((entity, mut same_kind_item, _)) => {
            same_kind_item.count += count;
            log(format!("Added {} to existing inventory item.", count));
            Some(entity)
        }
    }
}

fn create_inventory(world: &mut World, kind: &Arc<ItemKind>, count: u16) -> Entity {
    let mut item = Item::new(kind.clone());
    item.count = count;
    let entity = world
        .create_entity()
        .with(item)
        .with(InInventory::new())
        .id();
    log(format!("Created inventory entity - {:?}", entity));
    entity
}

fn reduce_count(world: &mut World, entity: Entity, count: u16) -> u16 {
    let unstack = {
        let item_kinds = world.read_global::<ItemKinds>();
        let items = world.read_component::<Item>();
        let item = items.get(entity).unwrap();

        let kind = item_kinds.get(&item.kind.id).unwrap();
        kind.stackable && item.count > count as u16
    };

    if unstack {
        let mut items = world.write_component::<Item>();
        let mut item = items.get_mut(entity).unwrap();
        item.count = item.count.saturating_sub(count as u16);
        log(format!("Delete {} of item - {:?}", count, entity));
        item.count
    } else {
        let items = world.read_component::<Item>();
        assert_eq!(count, items.get(entity).unwrap().count);
        0
    }
}

fn get_info(world: &World, entity: Entity) -> (Arc<ItemKind>, u16) {
    let items = world.read_component::<Item>();
    let item = items.get(entity).unwrap();
    (item.kind.clone(), item.count)
}
