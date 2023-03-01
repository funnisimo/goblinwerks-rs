use crate::app::{AppEvent, KeyEvent, VirtualKeyCode};
use std::collections::HashMap;
// use std::iter::Filter;

// / Provides information about user input.
// /
// / Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
// / only exist on Mac.
// /
// / State functions like [`InputApi::key`], [`InputApi::mouse_button`] and [`InputApi::mouse_pct`] always work.
// / The pressed/released event functions should be called only in the update function.
// /
// pub trait InputApi {
//     // keyboard state
//     /// return the current status of a key (true if pressed)
//     fn key(&self, key: VirtualKeyCode) -> bool;
//     /// return true if a key was pressed since last update.
//     fn key_pressed(&self, key: VirtualKeyCode) -> bool;
//     /// return true if a key was released since last update.
//     fn key_released(&self, key: VirtualKeyCode) -> bool;
//     /// return an iterator over all the keys that were pressed since last update.
//     // fn keys_pressed(&self) -> Keys;
//     /// return an iterator over all the keys that were released since last update.
//     // fn keys_released(&self) -> Keys;

//     // mouse
//     /// return the current status of a mouse button (true if pressed)
//     fn mouse_button(&self, num: usize) -> bool;
//     /// return true if a mouse button was pressed since last update.
//     fn mouse_button_pressed(&self, num: usize) -> bool;
//     /// return true if a mouse button was released since last update.
//     fn mouse_button_released(&self, num: usize) -> bool;
//     /// return the current mouse position on the screen in percent (0.0-1.0, 0.0-1.0)
//     /// give this to the console cell_pos method to get the cell the mouse is in
//     fn mouse_pct(&self) -> (f32, f32);

//     /// a mouse event occurred this frame
//     fn had_mouse_event(&self) -> bool;
//     /// a key event occurred this frame
//     fn had_key_event(&self) -> bool;

//     /// Whether the window close button was clicked
//     fn close_requested(&self) -> bool;
// }

/// Tracks all input events
pub struct AppInput {
    /// keys currently down
    kdown: HashMap<VirtualKeyCode, bool>,
    /// keys that were pressed this frame
    kpressed: HashMap<VirtualKeyCode, bool>,
    /// keys that were released this frame
    kreleased: HashMap<VirtualKeyCode, bool>,
    /// mouse buttons currently down
    mdown: HashMap<usize, bool>,
    /// mouse buttons pressed this frame
    mpressed: HashMap<usize, bool>,
    /// mouse buttons released this frame
    mreleased: HashMap<usize, bool>,
    /// user requested close of aplication
    close_request: bool,
    /// mouse position on screen in percent (0.0-1.0)
    mpos: (f32, f32),
    /// screen size in pixels
    screen_size: (f32, f32),
    /// the mouse offset from the screen pos
    mouse_offset: (f32, f32),

    /// all events that occurred this frame
    // events: Vec<AppEvent>,

    /// a mouse event occurred this frame
    mouse_event: bool,
    /// a keyboard event occurred this frame
    key_event: bool,
}

impl AppInput {
    /// Construct a new AppInput tracker with the given screen size and offset
    pub(crate) fn new(
        (screen_width, screen_height): (u32, u32),
        (x_offset, y_offset): (u32, u32),
    ) -> Self {
        Self {
            kdown: HashMap::new(),
            kpressed: HashMap::new(),
            kreleased: HashMap::new(),
            mdown: HashMap::new(),
            mpressed: HashMap::new(),
            mreleased: HashMap::new(),
            mpos: (0.0, 0.0),
            // text: Vec::new(),
            close_request: false,
            screen_size: (screen_width as f32, screen_height as f32),
            // con_size: (con_width as f32, con_height as f32),
            mouse_offset: (x_offset as f32, y_offset as f32),
            // last_pressed: None,
            // events: Vec::new(),
            mouse_event: false,
            key_event: false,
        }
    }

    /// handle a key down event
    fn on_key_down(&mut self, key: &KeyEvent) {
        if !self.key(key.key_code) {
            self.kpressed.insert(key.key_code, true);
            self.kdown.insert(key.key_code, true);
        }
    }

    /// handle a key up event
    fn on_key_up(&mut self, key: &KeyEvent) {
        self.kpressed.insert(key.key_code, false);
        self.kdown.insert(key.key_code, false);
        self.kreleased.insert(key.key_code, true);
    }

    /// handle a mouse down event
    fn on_mouse_down(&mut self, button: usize) {
        if !self.mouse_button(button) {
            self.mpressed.insert(button, true);
            self.mdown.insert(button, true);
        }
    }

    /// handle a mouse up event
    fn on_mouse_up(&mut self, button: usize) {
        self.mpressed.insert(button, false);
        self.mdown.insert(button, false);
        self.mreleased.insert(button, true);
    }

    /// a frame has ended
    pub(crate) fn on_frame_end(&mut self) {
        self.mpressed.clear();
        self.mreleased.clear();
        self.kreleased.clear();
        self.kpressed.clear();
        self.close_request = false;
        // self.events.clear();
        self.mouse_event = false;
        self.key_event = false;
    }

    /// an event occurred
    pub(crate) fn on_event(&mut self, event: &mut AppEvent) {
        // self.events.push(event.clone());

        match event {
            AppEvent::KeyDown(ref key) => {
                self.on_key_down(&key);
                self.key_event = true;
            }
            AppEvent::KeyUp(ref key) => {
                self.on_key_up(&key);
                self.key_event = true;
            }
            AppEvent::CharEvent(_ch) => {
                // match self.events.last_mut() {
                //     Some(AppEvent::KeyDown(ev)) => {
                //         ev.key = ch.to_string();
                //     }
                //     _ => {}
                // }
                self.key_event = true;
            }
            AppEvent::MousePos(ref mut pos) => {
                self.mpos = (
                    (pos.0 as f32 - self.mouse_offset.0) / self.screen_size.0,
                    (pos.1 as f32 - self.mouse_offset.1) / self.screen_size.1,
                );
                pos.0 = self.mpos.0;
                pos.1 = self.mpos.1;
                self.mouse_event = true;
            }
            AppEvent::MouseDown(ref mut mouse) => {
                mouse.pos.0 = (mouse.pos.0 as f32 - self.mouse_offset.0) / self.screen_size.0;
                mouse.pos.1 = (mouse.pos.1 as f32 - self.mouse_offset.1) / self.screen_size.1;
                self.on_mouse_down(mouse.button);
                self.mouse_event = true;
            }
            AppEvent::MouseUp(ref mut mouse) => {
                mouse.pos.0 = (mouse.pos.0 as f32 - self.mouse_offset.0) / self.screen_size.0;
                mouse.pos.1 = (mouse.pos.1 as f32 - self.mouse_offset.1) / self.screen_size.1;
                self.on_mouse_up(mouse.button);
                self.mouse_event = true;
            }
            AppEvent::CloseRequested => {
                self.close_request = true;
            }
            _ => (),
        }
    }

    /// change the size of the screen
    pub(crate) fn resize(
        &mut self,
        (screen_width, screen_height): (u32, u32),
        (x_offset, y_offset): (u32, u32),
    ) {
        self.screen_size = (screen_width as f32, screen_height as f32);
        // self.con_size = (con_width as f32, con_height as f32);
        self.mouse_offset = (x_offset as f32, y_offset as f32);
    }

    // InputAPI

    /// is this key currently down?
    pub fn key(&self, key_code: VirtualKeyCode) -> bool {
        matches!(self.kdown.get(&key_code), Some(&true))
    }
    /// was this key pressed this frame?
    pub fn key_pressed(&self, key_code: VirtualKeyCode) -> bool {
        matches!(self.kpressed.get(&key_code), Some(&true))
    }
    /// was this key released this frame?
    pub fn key_released(&self, key_code: VirtualKeyCode) -> bool {
        matches!(self.kreleased.get(&key_code), Some(&true))
    }

    /// Returns true if the given mouse button is currently pressed
    pub fn mouse_button(&self, num: usize) -> bool {
        matches!(self.mdown.get(&num), Some(&true))
    }

    /// Returns true if the given mouse button was pressed in this frame
    pub fn mouse_button_pressed(&self, num: usize) -> bool {
        matches!(self.mpressed.get(&num), Some(&true))
    }

    /// returns true if the given mouse button was released in this frame
    pub fn mouse_button_released(&self, num: usize) -> bool {
        matches!(self.mreleased.get(&num), Some(&true))
    }

    /// A mouse event occurred this frame
    pub fn had_mouse_event(&self) -> bool {
        self.mouse_event
    }

    /// A keyboard event occurred this frame
    pub fn had_key_event(&self) -> bool {
        self.key_event
    }

    /// returns the x,y percent of the mouse on the window - (0.0-1.0, 0.0-1.0)
    pub fn mouse_pct(&self) -> (f32, f32) {
        self.mpos
    }

    /// Returns true if user clicked the close button on the app window
    pub fn close_requested(&self) -> bool {
        self.close_request
    }
}
