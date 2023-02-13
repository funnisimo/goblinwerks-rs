use conapp::ecs::prelude::*;
use conapp::*;
use conui::screens::Choice;
use conui::ui::*;
use std::collections::HashMap;

struct MainScreen {
    ui: UI,
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

        Box::new(MainScreen { ui })
    }
}

impl Screen for MainScreen {
    fn input(&mut self, app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.ui.input(app, ev) {
            println!("- input={:?}", result);
            return result;
        }
        ScreenResult::Continue
    }

    fn message(
        &mut self,
        app: &mut AppContext,
        id: String,
        value: Option<MsgData>,
    ) -> ScreenResult {
        match id.as_str() {
            "CREATE" => {
                let world = &app.world;
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
                        .build(),
                );
            }
            "CREATE_ITEM" => {
                let text = self.ui.find_by_id("TEXT").unwrap();
                let id = match value {
                    None => {
                        text.set_text("Nothing");
                        return ScreenResult::Continue;
                    }
                    Some(v) => v.to_string(),
                };

                text.set_text(&id);

                let mut world = &mut app.world;

                let item_kinds = world.get_resource::<ItemKinds>().unwrap();
                let kind = item_kinds.get(&id).unwrap();
                if kind.stackable {
                    let mut query = world.query::<(&mut Item,)>();
                    match query.iter_mut(&mut world).find(|(item,)| item.kind == id) {
                        None => {
                            let entity = world.spawn((Item::new(&id),)).id();
                            console(format!("Created entity - {:?}", entity));
                        }
                        Some((mut same_kind_item,)) => {
                            same_kind_item.count += 1;
                            console("Added 1 to existing item.");
                        }
                    }
                } else {
                    let entity = world.spawn((Item::new(&id),)).id();
                    console(format!("Created entity - {:?}", entity));
                }
            }

            "DELETE" => {
                let world = &mut app.world;

                let mut query = world.query::<(Entity, &Item)>();

                let items: Vec<(String, MsgData)> = query
                    .iter(&world)
                    .map(|(entity, item)| (format!("1 {}", item.kind), entity.into()))
                    .collect();

                if items.len() > 0 {
                    return ScreenResult::Push(
                        Choice::builder("DELETE_ITEM")
                            .title("Delete which item?")
                            .items(items)
                            .class("blue-back")
                            .build(),
                    );
                } else {
                    // TODO - return ScreenResult::Push(MsgBox::builder("MSGBOX").title("Error").prompt("Nothing to delete.").ok().build());
                    return ScreenResult::Continue;
                }
            }

            "DELETE_ITEM" => {
                if let Some(val) = value {
                    let entity: Entity = val.try_into().unwrap();

                    let world = &mut app.world;

                    let unstack = {
                        let item_kinds = world.get_resource::<ItemKinds>().unwrap();
                        let item = world.entity(entity).get::<Item>().unwrap();

                        let kind = item_kinds.get(&item.kind).unwrap();
                        kind.stackable && item.count > 1
                    };

                    if unstack {
                        let mut entity_obj = world.entity_mut(entity);
                        let mut item = entity_obj.get_mut::<Item>().unwrap();
                        item.count -= 1;
                        console(format!("Delete 1 of item - {:?}", entity));
                    } else {
                        world.despawn(entity);
                        console(format!("Delete item - {:?}", entity));
                    }

                    // Fall through to update display
                }
            }

            "PICKUP" => {
                // return ScreenResult::Push(
                //     PickItem::builder("PICK_ITEM")
                //         .title("Drop which item(s)?")
                //         // .items()
                //         .class("blue-back")
                //         .build(),
                // );
            }
            "PICK_ITEM" => {
                self.ui.find_by_id("TEXT").unwrap().set_text("Nothing");
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
                // return ScreenResult::Push(
                //     PickItem::builder("PICK")
                //         .title("Drop which item(s)?")
                //         // .items()
                //         .class("blue-back")
                //         .build(),
                // );
            }
            "DROP_ITEM" => {
                self.ui.find_by_id("TEXT").unwrap().set_text("Nothing");
            }

            _ => return ScreenResult::Continue,
        }

        // update floor items
        let world = &mut app.world;
        let mut query = world.query::<(&Item,)>();
        let ids: Vec<String> = query
            .iter(&world)
            .map(|(item,)| format!("{} {}", item.count, item.kind))
            .collect();

        let floor = self.ui.find_by_id("FLOOR").unwrap();
        match ids.is_empty() {
            true => floor.set_text("Nothing."),
            false => floor.set_text(&ids.join("\n")),
        }

        // update inventory items

        console("Update UI");
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        self.ui.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Inventory Example")
        .file("resources/styles.css", Box::new(load_stylesheet_data))
        // .file("resources/items.toml", Box::new(load_item_kinds_data))
        .vsync(false)
        .build();

    app.run_with(Box::new(|app: &mut AppContext| {
        console("STARTUP");
        init_item_kinds(app);

        MainScreen::new()
    }));
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
    kinds: HashMap<String, ItemKind>,
}

impl ItemKinds {
    fn new() -> Self {
        ItemKinds {
            kinds: HashMap::new(),
        }
    }

    fn insert(&mut self, kind: ItemKind) {
        self.kinds.insert(kind.id.clone(), kind);
    }

    fn get(&self, id: &str) -> Option<&ItemKind> {
        self.kinds.get(id)
    }

    fn iter(&self) -> impl Iterator<Item = &ItemKind> {
        self.kinds.values()
    }
}

fn init_item_kinds(app: &mut AppContext) {
    let world = &mut app.world;

    let mut item_kinds = ItemKinds::new();
    item_kinds.insert(ItemKind::new("TACO", "taco", true));
    item_kinds.insert(ItemKind::new("BURRITO", "burrito", true));
    item_kinds.insert(ItemKind::new("SOMBRERO", "sombrero", false));
    item_kinds.insert(ItemKind::new("CACTUS", "cactus", false));

    world.insert_resource(item_kinds);
}

#[derive(Component, Clone)]
struct Item {
    kind: String,
    count: u16,
}

impl Item {
    fn new(id: &str) -> Self {
        Item {
            kind: id.to_string(),
            count: 1,
        }
    }
}
