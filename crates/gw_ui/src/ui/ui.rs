use super::*;
use crate::css::STYLES;
use gw_app::log;
use gw_app::AppContext;
use gw_app::AppEvent;
use gw_app::Buffer;
use gw_app::Console;
use gw_app::KeyEvent;
use gw_app::MsgData;
use gw_app::Point;
use gw_app::Screen;
use gw_app::ScreenResult;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub type UiActionFn = dyn Fn(&Element, &Element) -> Option<UiAction>;

pub enum UiAction {
    Message(String, Option<MsgData>), // message to send to appcontext
    Activate(String),                 // Id of element to activate
    Focus(String),                    // Focus on this ID
    NextFocus,
    PrevFocus,
    Stop,
    Screen(Box<dyn Screen>),
}

impl UiAction {
    pub fn message(id: String, data: Option<MsgData>) -> Box<UiActionFn> {
        let info = Rc::new((id, data));
        Box::new(move |_, _| Some(UiAction::Message(info.0.clone(), info.1.clone())))
    }

    pub fn activate(id: String) -> Box<UiActionFn> {
        let info = Rc::new((id,));
        Box::new(move |_, _| Some(UiAction::Activate(info.0.clone())))
    }

    pub fn focus(id: String) -> Box<UiActionFn> {
        let info = Rc::new((id,));
        Box::new(move |_, _| Some(UiAction::Focus(info.0.clone())))
    }

    pub fn focus_next() -> Box<UiActionFn> {
        Box::new(|_, _| Some(UiAction::NextFocus))
    }

    pub fn focus_prev() -> Box<UiActionFn> {
        Box::new(|_, _| Some(UiAction::PrevFocus))
    }

    pub fn stop() -> Box<UiActionFn> {
        Box::new(|_, _| Some(UiAction::Stop))
    }
}

impl Debug for UiAction {
    /// Shows the name of the enum value
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UiAction::Message(id, data) => format!("Message({},{:?}", id, data),
                UiAction::Activate(id) => format!("Activate({})", id),
                UiAction::Focus(id) => format!("Focus({})", id),
                UiAction::NextFocus => "NextFocus".to_owned(),
                UiAction::PrevFocus => "PrevFocus".to_owned(),
                UiAction::Stop => "Stop".to_owned(),
                UiAction::Screen(_) => "Screen(..)".to_owned(),
            }
        )
    }
}

impl PartialEq for UiAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UiAction::Message(id1, data1), UiAction::Message(id2, data2)) => {
                id1 == id2 && data1 == data2
            }
            (UiAction::Activate(id1), UiAction::Activate(id2)) => id1 == id2,
            (UiAction::Focus(id1), UiAction::Focus(id2)) => id1 == id2,
            (UiAction::NextFocus, UiAction::NextFocus) => true,
            (UiAction::PrevFocus, UiAction::PrevFocus) => true,
            (UiAction::Stop, UiAction::Stop) => true,
            (UiAction::Screen(_), UiAction::Screen(_)) => true,
            _ => false,
        }
    }
}

pub struct UI {
    root: Element,
    pub(crate) focus_order: Vec<Element>,
    pub(crate) last_mouse: RefCell<Point>,
    pub(crate) console: Console,
}

impl UI {
    fn new(console: Console, root: Element) -> Self {
        let mut ui = UI {
            root,
            console,
            focus_order: Vec::new(),
            last_mouse: RefCell::new(Point::new(-1, -1)),
        };

        ui.update_styles();

        // now collect focus order
        let mut has_focus = false;
        ui.root.every_element(&mut |el: &Element| {
            if el.can_focus() {
                ui.focus_order.push(el.clone());
                if el.has_prop("focus") {
                    if has_focus {
                        // We already have a focused element
                        el.remove_prop("focus");
                    }
                    has_focus = true;
                    println!("Focus on = {}", &el);
                }
            }
        });

        println!(
            "UI Focus order - {:?}",
            ui.focus_order
                .iter()
                .map(|el| match el.id().as_ref() {
                    None => "!".to_string(),
                    Some(id) => id.clone(),
                },)
                .collect::<Vec<String>>()
        );

        if has_focus == false && ui.focus_order.len() > 0 {
            println!("Focus on = {}", &ui.focus_order[0]);
            ui.focus_on(&ui.focus_order[0]);
        }

        ui
    }

    pub fn update_styles(&mut self) {
        // now assign all the styles...
        let sheet = STYLES.lock().unwrap();
        self.root.setup_style(&*sheet);
    }

    pub fn root(&self) -> Element {
        self.root.clone()
    }

    pub fn is_full_screen(&self) -> bool {
        self.console.is_full_screen()
    }

    pub fn mouse_pos(&self, screen_pct: (f32, f32)) -> Option<(u32, u32)> {
        match self.console.mouse_pos(screen_pct) {
            None => None,
            Some(con_pos) => Some((con_pos.0.floor() as u32, con_pos.1.floor() as u32)),
        }
    }

    // fn clickable_at(&self, point: Point) -> Option<Element> {
    //     self.root
    //         .leaf_matching(|n| n.contains(point) && n.borrow().click)
    // }

    // pub(crate) fn handle_click(&self, ctx: &mut  AppContext, point: Point) -> ScreenResult {
    //     if let Some(el) = self.clickable_at(point) {
    //         return el.fire_click(ctx, self);
    //     }
    //     ScreenResult::Continue
    // }

    pub fn focus(&self, id: &str) {
        if let Some(el) = self.find_by_id(id) {
            self.focus_on(&el);
        }
    }

    pub fn focus_on(&self, el: &Element) {
        self.every_element(&mut |el| el.remove_prop("hover")); // clear all hover
        for el in self.focus_order.iter() {
            el.remove_prop("focus");
        }
        el.add_prop("focus");
    }

    pub fn focused(&self) -> Option<Element> {
        self.focus_order
            .iter()
            .find_map(|el| match el.has_prop("focus") {
                false => None,
                true => Some(el.clone()),
            })
    }

    fn next_focus(&mut self) {
        if self.focus_order.len() == 0 {
            return;
        }
        let last_idx = self.focus_order.len() - 1;

        let focus = match self.focus_order.iter().position(|el| el.has_prop("focus")) {
            Some(pos) if pos < last_idx => &self.focus_order[pos + 1],
            _ => &self.focus_order[0],
        };

        self.focus_on(focus);
    }

    fn prev_focus(&mut self) {
        if self.focus_order.len() == 0 {
            return;
        }
        let last_idx = self.focus_order.len() - 1;

        let focus = match self.focus_order.iter().position(|el| el.has_prop("focus")) {
            Some(pos) if pos == 0 => &self.focus_order[last_idx],
            Some(pos) => &self.focus_order[pos - 1],
            None => &self.focus_order[last_idx],
        };

        self.focus_on(focus);
    }

    fn update_hover(&self, point: Point) {
        self.root.update_hover(point);
    }

    pub fn every_element<F>(&self, func: &mut F)
    where
        F: FnMut(&Element) -> (),
    {
        self.root.every_element(func);
    }

    pub fn find_by_id(&self, id: &str) -> Option<Element> {
        self.root.find_by_id(id)
    }

    fn do_action(
        &mut self,
        app: &mut AppContext,
        action: Option<UiAction>,
    ) -> Option<ScreenResult> {
        log("do_action");
        match action {
            None => {}
            Some(UiAction::Activate(id)) => {
                if let Some(el) = self.find_by_id(&id) {
                    return self.do_action(app, el.handle_activate(&self.root));
                }
            }
            Some(UiAction::Focus(id)) => {
                self.every_element(&mut |el| el.remove_prop("hover")); // clear all hover
                self.focus(&id);
            }
            Some(UiAction::NextFocus) => {
                self.every_element(&mut |el| el.remove_prop("hover")); // clear all hover
                self.next_focus();
            }
            Some(UiAction::PrevFocus) => {
                self.every_element(&mut |el| el.remove_prop("hover")); // clear all hover
                self.prev_focus();
            }
            Some(UiAction::Message(id, data)) => {
                app.send_message(&id, data);
            }
            Some(UiAction::Stop) => {
                return Some(ScreenResult::Continue);
            }
            Some(UiAction::Screen(screen)) => {
                println!("PUSH SCREEN FROM UI");
                return Some(ScreenResult::Push(screen));
            }
        }
        None
    }

    pub fn input(&mut self, app: &mut AppContext, ev: &AppEvent) -> Option<ScreenResult> {
        let screen_pos = app.input().mouse_pct();
        if let Some(mouse_pos) = self.console.mouse_pos(screen_pos) {
            let mouse_pt: Point = mouse_pos.into();
            match ev {
                AppEvent::MouseDown(_) => {
                    let action = self.handle_click(mouse_pt);
                    if let Some(result) = self.do_action(app, action) {
                        return Some(result);
                    }
                }
                AppEvent::MousePos(_) => {
                    if mouse_pt != *self.last_mouse.borrow() {
                        *self.last_mouse.borrow_mut() = mouse_pt;
                        // println!("mouse move - {:?}", mouse_pt);
                        if let Some(focused) = self.root.update_hover(mouse_pt) {
                            self.every_element(&mut |el| el.remove_prop("focus"));
                            focused.add_prop("focus");
                        }
                    }
                }
                _ => {}
            }
        }
        match ev {
            AppEvent::KeyDown(key_down) => {
                let action = self.handle_key(key_down);
                if let Some(result) = self.do_action(app, action) {
                    println!("- ui::key_down = {:?}", result);
                    return Some(result);
                }
            }
            _ => {}
        }
        None
    }

    pub(crate) fn handle_key(&mut self, key: &KeyEvent) -> Option<UiAction> {
        let el = self.focused().unwrap_or(self.root.clone());
        println!("- handle key={:?}, el={:?}", key, el.id());
        el.handle_key(&self.root, key)
    }

    pub(crate) fn handle_click(&mut self, mouse_pt: Point) -> Option<UiAction> {
        println!("mouse click - {:?}", mouse_pt);
        self.root.clone().handle_click(&self.root, mouse_pt)
    }

    pub fn draw(&self, buf: &mut Buffer) {
        buf.clear(true, true, true);
        self.root.draw(buf);
    }

    pub fn render(&mut self, app: &mut AppContext) {
        self.console.buffer_mut().clear(true, true, true);
        self.root.draw(self.console.buffer_mut());
        self.console.render(app);
    }

    pub fn dump(&self) {
        dump_element(&self.root);
    }
}

impl Debug for UI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut log = f.debug_struct("UI");
        log.field(
            "focus",
            &self
                .focus_order
                .iter()
                .map(|el| match el.id().as_ref() {
                    None => "UI".to_string(),
                    Some(id) => id.clone(),
                })
                .collect::<Vec<String>>(),
        );

        self.every_element(&mut |el| {
            let tag_str = el.tag();
            let key = match el.id().as_ref() {
                None => format!("{}", tag_str),
                Some(id) => format!("{}#{}", tag_str, id),
            };

            log.field(&key, &el);
        });

        log.finish()
    }
}

pub fn page<F>(page_size: (u32, u32), font: &str, setup: F) -> UI
where
    F: FnOnce(&mut BodyBuilder),
{
    println!("<< NEW PAGE >>");
    let body = Body::new(page_size, setup);

    let size = body.size().unwrap();
    let console = Console::new(size.0, size.1, font);
    UI::new(console, body)
}

pub fn dialog<F>(page_size: (u32, u32), font: &str, setup: F) -> UI
where
    F: FnOnce(&mut DialogBuilder),
{
    let dialog = Dialog::new(page_size, setup);

    let base_size = dialog.size().unwrap(); // guaranteed to be set by constructor
    let size = (base_size.0.min(page_size.0), base_size.1.min(page_size.1));

    let page_pos = match dialog.pos() {
        None => {
            let left = match dialog.anchor() {
                Some(Align::Min) => 0,
                Some(Align::Center) | None => {
                    println!("dialog pos = root_size={:?}, size={:?}", page_size, size);
                    page_size.0.saturating_sub(size.0) as i32 / 2
                }
                Some(Align::Max) => page_size.0.saturating_sub(size.0) as i32,
            };

            let right = match dialog.vanchor() {
                Some(Align::Min) => 0,
                Some(Align::Center) | None => page_size.1.saturating_sub(size.1) as i32 / 2,
                Some(Align::Max) => page_size.1.saturating_sub(size.1) as i32,
            };
            (left, right)
        }
        Some(pos) => (
            pos.0.clamp(0, (page_size.0 - size.0) as i32),
            pos.1.clamp(0, (page_size.1 - size.1) as i32),
        ),
    };

    dialog.set_pos(0, 0);

    let mut console = Console::new(size.0, size.1, font);

    // set extents
    let left = page_pos.0 as f32 / page_size.0 as f32;
    let top = page_pos.1 as f32 / page_size.1 as f32;
    let right = left + size.0 as f32 / page_size.0 as f32;
    let bottom = top + size.1 as f32 / page_size.1 as f32;

    console.set_extents(left, top, right, bottom);

    UI::new(console, dialog)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_page() {
        let ui = page((80, 50), "DEFAULT", |body| {
            body.align(Align::Center).valign(Align::Center);

            Text::new(body, |txt| {
                txt.text("Hello World");
            });
        });

        assert_eq!(ui.console.size(), (80, 50));
        assert_eq!(ui.console.font_name(), "DEFAULT");
        assert_eq!(ui.root.tag(), "body");
        let text = ui.root.first_child().unwrap();
        assert_eq!(text.tag(), "text");
        assert_eq!(text.pos().unwrap(), (34, 24)); // center, middle
        assert_eq!(text.size().unwrap(), (11, 1));
    }

    #[test]
    fn basic_dialog() {
        let ui = dialog((80, 50), "DEFAULT", |dialog| {
            Text::new(dialog, |txt| {
                txt.text("Hello World");
            });
        });

        assert_eq!(ui.console.size(), (11, 1));
        assert_eq!(ui.console.font_name(), "DEFAULT");
        assert_eq!(ui.root.tag(), "dialog");
        let text = ui.root.first_child().unwrap();
        assert_eq!(text.tag(), "text");
        assert_eq!(text.pos().unwrap(), (0, 0)); // only thing there
        assert_eq!(text.size().unwrap(), (11, 1));
    }
}
