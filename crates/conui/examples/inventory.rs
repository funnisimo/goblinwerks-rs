use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;
use conapp::screen::DataConvertError;
use conapp::*;
use conui::css::*;
use conui::screens::{Choice, MultiChoice};
use conui::ui::*;
use std::collections::HashMap;
use std::sync::Arc;

fn entity_from<V: TryInto<u64>>(v: V) -> Result<Entity, DataConvertError> {
    let bits = match v.try_into() {
        Err(_) => return Err(DataConvertError::WrongType),
        Ok(v) => v,
    };
    Ok(Entity::from_bits(bits))
}

struct MainScreen {
    ui: UI,
    schedule: Schedule,
    world: World,
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
            schedule: Schedule::default(),
            world: World::new(),
        })
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, _app: &mut AppContext) {
        init_item_kinds(&mut self.world);
        self.ui.update_styles();
    }

    fn input(&mut self, app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.ui.input(app, ev) {
            println!("- input={:?}", result);
            return result;
        }
        ScreenResult::Continue
    }

    fn message(
        &mut self,
        _app: &mut AppContext,
        id: String,
        value: Option<MsgData>,
    ) -> ScreenResult {
        match id.as_str() {
            "CREATE" => {
                let world = &self.world;
                let item_kinds = world.get_resource::<ItemKinds>().unwrap();

                let items: Vec<(String, MsgData)> = item_kinds
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
                let world = &mut self.world;

                let item_kinds = world.get_resource::<ItemKinds>().unwrap();
                let kind = item_kinds.get(&kind_id).unwrap();

                if stack_floor(world, &kind, 1).is_none() {
                    create_floor(world, &kind, 1);
                }
            }

            "DELETE" => {
                let world = &mut self.world;

                let mut query = world.query_filtered::<(Entity, &Item), Without<InInventory>>();

                let items: Vec<(String, Key, u16)> = query
                    .iter(&world)
                    .map(|(entity, item)| {
                        (
                            format!("{} {}", item.count, item.kind.name),
                            entity.to_bits().into(),
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
                if let Some(MsgData::Map(map)) = value {
                    for (key, val) in map {
                        let entity: Entity = entity_from(key).unwrap();
                        let count: i32 = val.try_into().unwrap();

                        let world = &mut self.world;
                        if reduce_count(world, entity, count as u16) == 0 {
                            world.despawn(entity);
                            log(format!("Delete item - {:?}", entity));
                        }
                    }
                }
            }

            "PICKUP" => {
                let world = &mut self.world;

                let mut query = world.query_filtered::<(Entity, &Item), Without<InInventory>>();

                let items: Vec<(String, Key, u16)> = query
                    .iter(&world)
                    .map(|(entity, item)| {
                        (
                            format!("{} {}", item.count, item.kind.name),
                            entity.to_bits().into(),
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
                if let Some(MsgData::Map(map)) = value {
                    for (key, val) in map {
                        let entity: Entity = entity_from(key).unwrap();
                        let count: i32 = val.try_into().unwrap();

                        let world = &mut self.world;

                        let (kind_id, has_count) = get_info(world, entity);

                        match has_count == count as u16 {
                            true => match stack_inventory(world, &kind_id, count as u16) {
                                None => {
                                    world.entity_mut(entity).insert(InInventory::new());
                                }
                                Some(_) => {
                                    world.despawn(entity);
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
                let world = &mut self.world;

                let mut query = world.query_filtered::<(Entity, &Item), With<InInventory>>();

                let items: Vec<(String, Key, u16)> = query
                    .iter(&world)
                    .map(|(entity, item)| {
                        (
                            format!("{} {}", item.count, item.kind.id),
                            entity.to_bits().into(),
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
                if let Some(MsgData::Map(map)) = value {
                    for (key, val) in map {
                        let entity: Entity = entity_from(key).unwrap();
                        let count: i32 = val.try_into().unwrap();

                        let world = &mut self.world;
                        let (kind, has_count) = get_info(world, entity);

                        match has_count == count as u16 {
                            true => {
                                log("Removing whole inventory item");
                                if stack_floor(world, &kind, count as u16).is_some() {
                                    world.despawn(entity);
                                } else {
                                    world.entity_mut(entity).remove::<InInventory>();
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
        let world = &mut self.world;
        let mut query = world.query_filtered::<(&Item,), Without<InInventory>>();
        let ids: Vec<String> = query
            .iter(&world)
            .map(|(item,)| format!("{} {}", item.count, item.kind.name))
            .collect();

        let floor = self.ui.find_by_id("FLOOR").unwrap();
        match ids.is_empty() {
            true => floor.set_text("Nothing."),
            false => floor.set_text(&ids.join("\n")),
        }

        // update inventory items
        let mut query = world.query_filtered::<(&Item,), With<InInventory>>();
        let ids: Vec<String> = query
            .iter(&world)
            .map(|(item,)| format!("{} {}", item.count, item.kind.name))
            .collect();

        let floor = self.ui.find_by_id("INVENTORY").unwrap();
        match ids.is_empty() {
            true => floor.set_text("Nothing."),
            false => floor.set_text(&ids.join("\n")),
        }

        log("Update UI");
        ScreenResult::Continue
    }

    fn update(&mut self, _app: &mut AppContext, _frame_time_ms: f64) -> ScreenResult {
        let world = &mut self.world;
        self.schedule.run_once(world);
        world.clear_trackers();

        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        self.ui.render(app);
    }

    fn teardown(&mut self, _app: &mut AppContext) {}
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Inventory Example")
        .file(
            "resources/styles.css",
            Box::new(|data, app| {
                load_stylesheet_data(data, app)?;
                log("STYLES LOADED!");
                Ok(())
            }),
        )
        .vsync(false)
        .build();

    app.run_screen(MainScreen::new());
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

#[derive(Resource, Clone)]
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

fn init_item_kinds(world: &mut World) {
    let mut item_kinds = ItemKinds::new();
    item_kinds.insert(ItemKind::new("TACO", "taco", true));
    item_kinds.insert(ItemKind::new("BURRITO", "burrito", true));
    item_kinds.insert(ItemKind::new("SOMBRERO", "sombrero", false));
    item_kinds.insert(ItemKind::new("CACTUS", "cactus", false));

    world.insert_resource(item_kinds);
}

#[derive(Component, Clone)]
struct Item {
    kind: Arc<ItemKind>,
    count: u16,
}

impl Item {
    fn new(kind: Arc<ItemKind>) -> Self {
        Item { kind, count: 1 }
    }
}

#[derive(Component, Clone)]
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

    let mut query = world.query_filtered::<(&mut Item, Entity), Without<InInventory>>();
    match query
        .iter_mut(world)
        .find(|(item, _)| item.kind.id == kind.id)
    {
        None => None,
        Some((mut same_kind_item, entity)) => {
            same_kind_item.count += count;
            log(format!("Added {} to existing floor item.", count));
            Some(entity)
        }
    }
}

fn create_floor(world: &mut World, kind: &Arc<ItemKind>, count: u16) -> Entity {
    let mut item = Item::new(kind.clone());
    item.count = count;
    let entity = world.spawn((item,)).id();
    log(format!("Created floor entity - {:?}", entity));
    entity
}

fn stack_inventory(world: &mut World, kind: &Arc<ItemKind>, count: u16) -> Option<Entity> {
    if !kind.stackable {
        return None;
    }

    let mut query = world.query_filtered::<(&mut Item, Entity), With<InInventory>>();
    match query
        .iter_mut(world)
        .find(|(item, _)| item.kind.id == kind.id)
    {
        None => None,
        Some((mut same_kind_item, entity)) => {
            same_kind_item.count += count;
            log(format!("Added {} to existing inventory item.", count));
            Some(entity)
        }
    }
}

fn create_inventory(world: &mut World, kind: &Arc<ItemKind>, count: u16) -> Entity {
    let mut item = Item::new(kind.clone());
    item.count = count;
    let entity = world.spawn((item, InInventory::new())).id();
    log(format!("Created inventory entity - {:?}", entity));
    entity
}

fn reduce_count(world: &mut World, entity: Entity, count: u16) -> u16 {
    let unstack = {
        let item_kinds = world.get_resource::<ItemKinds>().unwrap();
        let item = world.entity(entity).get::<Item>().unwrap();

        let kind = item_kinds.get(&item.kind.id).unwrap();
        kind.stackable && item.count > count as u16
    };

    if unstack {
        let mut entity_obj = world.entity_mut(entity);
        let mut item = entity_obj.get_mut::<Item>().unwrap();
        item.count = item.count.saturating_sub(count as u16);
        log(format!("Delete {} of item - {:?}", count, entity));
        item.count
    } else {
        assert_eq!(count, world.entity(entity).get::<Item>().unwrap().count);
        0
    }
}

fn get_info(world: &World, entity: Entity) -> (Arc<ItemKind>, u16) {
    let item = world.entity(entity).get::<Item>().unwrap();
    (item.kind.clone(), item.count)
}
