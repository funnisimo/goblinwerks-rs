// wasm-unknown-unknown
#[cfg(target_arch = "wasm32")]
#[path = "web_app.rs"]
pub mod sys;

#[cfg(target_arch = "wasm32")]
#[path = "web_fs.rs"]
pub mod fs;

// // NOT wasm-unknown-unknown
// #[cfg(not(target_arch = "wasm32"))]
// extern crate glutin;

// #[cfg(not(target_arch = "wasm32"))]
// extern crate time;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native_app.rs"]
/// main application struct
pub mod sys;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native_fs.rs"]
/// filesystem api
pub mod fs;

pub use self::fs::*;
pub use self::sys::*;

#[derive(Clone, Debug)]
/// game window configuration
pub struct AppConfig {
    /// the window title (only visible on native target)
    pub title: String,
    /// the desired fps
    pub fps: u32,
    /// the window/canvas size in pixels
    pub size: (u32, u32),
    /// sync frames with screen frequency (can only be disabled on native target)
    pub vsync: bool,
    // /// start the program without actually creating a window, for test purposes
    // pub headless: bool,
    /// start in full screen (native target only)
    pub fullscreen: bool,
    /// whether user can resize the window (native target only)
    pub resizable: bool,
    /// whether the mouse cursor is visible while in the window
    pub show_cursor: bool,
    /// whether clicking on the window close button exits the program or sends a CloseRequested event
    pub intercept_close_request: bool,
}

impl AppConfig {
    pub fn new<T: Into<String>>(title: T, size: (u32, u32)) -> AppConfig {
        AppConfig {
            title: title.into(),
            size,
            ..Default::default()
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            title: "conapp".into(),
            size: (50, 30),
            fps: 60,
            vsync: true,
            // headless: false,
            fullscreen: false,
            resizable: true,
            show_cursor: true,
            intercept_close_request: false,
        }
    }
}

/// keyboard and mouse events
pub mod events {
    use std::fmt;
    use std::hash::{Hash, Hasher};

    pub use winit::event::VirtualKeyCode;

    #[derive(Debug, Clone)]
    /// data associated with a mouse button press/release event
    pub struct MouseButtonEvent {
        /// the button number (0=left, 1=middle, 2=right, ...)
        pub button: usize,
        pub pos: (f32, f32),
    }

    #[derive(Clone, Eq)]
    /// data associated with a key press or release event
    /// Possible values for the scancode/virtual key code can be found in unrust/uni-app's `translate_scan_code`
    /// [function](https://github.com/unrust/uni-app/blob/41246b070567e3267f128fff41ededf708149d60/src/native_keycode.rs#L160).
    /// Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
    /// only exist on Mac.
    pub struct KeyEvent {
        /// virtual key code string : top left letter is "KeyQ" on qwerty, "KeyA" on azerty
        pub code: String,
        /// text of key
        pub key: String,
        /// virtual key code
        pub key_code: VirtualKeyCode,
        /// whether a shift key is pressed
        pub shift: bool,
        /// whether a control key is pressed
        pub ctrl: bool,
        /// whether an alt key is pressed
        pub alt: bool,
    }

    impl fmt::Debug for KeyEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}{}{}{}({})",
                if self.shift { "shift+" } else { "" },
                if self.ctrl { "ctrl+" } else { "" },
                if self.alt { "alt+" } else { "" },
                self.code,
                self.key,
            )
        }
    }

    use super::translate_virtual_key;

    impl From<VirtualKeyCode> for KeyEvent {
        fn from(vkc: VirtualKeyCode) -> Self {
            KeyEvent {
                code: translate_virtual_key(vkc).to_owned(),
                key: translate_virtual_key(vkc).to_owned(),
                key_code: vkc,
                shift: false,
                ctrl: false,
                alt: false,
            }
        }
    }

    impl From<(VirtualKeyCode, bool)> for KeyEvent {
        fn from((vkc, shift): (VirtualKeyCode, bool)) -> Self {
            KeyEvent {
                code: translate_virtual_key(vkc).to_owned(),
                key: translate_virtual_key(vkc).to_owned(),
                key_code: vkc,
                shift,
                ctrl: false,
                alt: false,
            }
        }
    }

    impl From<(VirtualKeyCode, bool, bool)> for KeyEvent {
        fn from((vkc, shift, ctrl): (VirtualKeyCode, bool, bool)) -> Self {
            KeyEvent {
                code: translate_virtual_key(vkc).to_owned(),
                key: translate_virtual_key(vkc).to_owned(),
                key_code: vkc,
                shift,
                ctrl,
                alt: false,
            }
        }
    }

    impl From<(VirtualKeyCode, bool, bool, bool)> for KeyEvent {
        fn from((vkc, shift, ctrl, alt): (VirtualKeyCode, bool, bool, bool)) -> Self {
            KeyEvent {
                code: translate_virtual_key(vkc).to_owned(),
                key: translate_virtual_key(vkc).to_owned(),
                key_code: vkc,
                shift,
                ctrl,
                alt,
            }
        }
    }

    impl PartialEq for KeyEvent {
        fn eq(&self, other: &Self) -> bool {
            self.key_code == other.key_code
                && self.shift == other.shift
                && self.ctrl == other.ctrl
                && self.alt == other.alt
        }
    }

    impl Hash for KeyEvent {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.key_code.hash(state);
            self.shift.hash(state);
            self.ctrl.hash(state);
            self.alt.hash(state);
        }
    }
}

pub use events::*;

#[derive(Debug, Clone)]
/// window event types
pub enum AppEvent {
    /// mouse button press
    MouseDown(MouseButtonEvent),
    /// mouse button release
    MouseUp(MouseButtonEvent),
    /// keyboard press
    KeyDown(KeyEvent),
    /// keyboard release
    KeyUp(KeyEvent),
    /// text input events
    CharEvent(char),
    /// window resize
    Resized((u32, u32)),
    /// mouse cursor position in pixels from the window top-left
    MousePos((f32, f32)),
    /// a file has been dropped on the game window. Get it with `App.get_dropped_file`
    FileDropped(String),
    /// window close button was pressed and [`AppConfig.intercept_close_request`] is true
    CloseRequested,

    /// window and gl context are ready
    Ready,
    /// app is in background, window and gl invalid
    Suspended,
}

// use uni_app::*;

// pub use uni_app::{
//     now, App, AppConfig, AppEvent, File, FileSystem, KeyDownEvent, KeyEvent, MouseButtonEvent,
//     VirtualKeyCode,
// };

pub(self) fn translate_virtual_key(c: VirtualKeyCode) -> &'static str {
    match c {
        VirtualKeyCode::Key1 => "Digit1",
        VirtualKeyCode::Key2 => "Digit2",
        VirtualKeyCode::Key3 => "Digit3",
        VirtualKeyCode::Key4 => "Digit4",
        VirtualKeyCode::Key5 => "Digit5",
        VirtualKeyCode::Key6 => "Digit6",
        VirtualKeyCode::Key7 => "Digit7",
        VirtualKeyCode::Key8 => "Digit8",
        VirtualKeyCode::Key9 => "Digit9",
        VirtualKeyCode::Key0 => "Digit0",
        VirtualKeyCode::A => "KeyA",
        VirtualKeyCode::B => "KeyB",
        VirtualKeyCode::C => "KeyC",
        VirtualKeyCode::D => "KeyD",
        VirtualKeyCode::E => "KeyE",
        VirtualKeyCode::F => "KeyF",
        VirtualKeyCode::G => "KeyG",
        VirtualKeyCode::H => "KeyH",
        VirtualKeyCode::I => "KeyI",
        VirtualKeyCode::J => "KeyJ",
        VirtualKeyCode::K => "KeyK",
        VirtualKeyCode::L => "KeyL",
        VirtualKeyCode::M => "KeyM",
        VirtualKeyCode::N => "KeyN",
        VirtualKeyCode::O => "KeyO",
        VirtualKeyCode::P => "KeyP",
        VirtualKeyCode::Q => "KeyQ",
        VirtualKeyCode::R => "KeyR",
        VirtualKeyCode::S => "KeyS",
        VirtualKeyCode::T => "KeyT",
        VirtualKeyCode::U => "KeyU",
        VirtualKeyCode::V => "KeyV",
        VirtualKeyCode::W => "KeyW",
        VirtualKeyCode::X => "KeyX",
        VirtualKeyCode::Y => "KeyY",
        VirtualKeyCode::Z => "KeyZ",
        VirtualKeyCode::Escape => "Escape",
        VirtualKeyCode::F1 => "F1",
        VirtualKeyCode::F2 => "F2",
        VirtualKeyCode::F3 => "F3",
        VirtualKeyCode::F4 => "F4",
        VirtualKeyCode::F5 => "F5",
        VirtualKeyCode::F6 => "F6",
        VirtualKeyCode::F7 => "F7",
        VirtualKeyCode::F8 => "F8",
        VirtualKeyCode::F9 => "F9",
        VirtualKeyCode::F10 => "F10",
        VirtualKeyCode::F11 => "F11",
        VirtualKeyCode::F12 => "F12",
        VirtualKeyCode::F13 => "F13",
        VirtualKeyCode::F14 => "F14",
        VirtualKeyCode::F15 => "F15",
        VirtualKeyCode::F16 => "F16",
        VirtualKeyCode::F17 => "F17",
        VirtualKeyCode::F18 => "F18",
        VirtualKeyCode::F19 => "F19",
        VirtualKeyCode::F20 => "F20",
        VirtualKeyCode::F21 => "F21",
        VirtualKeyCode::F22 => "F22",
        VirtualKeyCode::F23 => "F23",
        VirtualKeyCode::F24 => "F24",
        VirtualKeyCode::Snapshot => "",
        VirtualKeyCode::Scroll => "ScrollLock",
        VirtualKeyCode::Pause => "Pause",
        VirtualKeyCode::Insert => "Insert",
        VirtualKeyCode::Home => "Home",
        VirtualKeyCode::Delete => "Delete",
        VirtualKeyCode::End => "End",
        VirtualKeyCode::PageDown => "PageDown",
        VirtualKeyCode::PageUp => "PageUp",
        VirtualKeyCode::Left => "ArrowLeft",
        VirtualKeyCode::Up => "ArrowUp",
        VirtualKeyCode::Right => "ArrowRight",
        VirtualKeyCode::Down => "ArrowDown",
        VirtualKeyCode::Back => "Backspace",
        VirtualKeyCode::Return => "Enter",
        VirtualKeyCode::Space => "Space",
        VirtualKeyCode::Compose => "",
        VirtualKeyCode::Numlock => "NumLock",
        VirtualKeyCode::Numpad0 => "Numpad0",
        VirtualKeyCode::Numpad1 => "Numpad1",
        VirtualKeyCode::Numpad2 => "Numpad2",
        VirtualKeyCode::Numpad3 => "Numpad3",
        VirtualKeyCode::Numpad4 => "Numpad4",
        VirtualKeyCode::Numpad5 => "Numpad5",
        VirtualKeyCode::Numpad6 => "Numpad6",
        VirtualKeyCode::Numpad7 => "Numpad7",
        VirtualKeyCode::Numpad8 => "Numpad8",
        VirtualKeyCode::Numpad9 => "Numpad9",
        VirtualKeyCode::AbntC1 => "",
        VirtualKeyCode::AbntC2 => "",
        VirtualKeyCode::NumpadAdd => "NumpadAdd",
        VirtualKeyCode::Apostrophe => "Apostrophe",
        VirtualKeyCode::Apps => "",
        VirtualKeyCode::Asterisk => "Star",
        VirtualKeyCode::At => "",
        VirtualKeyCode::Ax => "",
        VirtualKeyCode::Backslash => "Backslash",
        VirtualKeyCode::Calculator => "",
        VirtualKeyCode::Capital => "CapsLock",
        VirtualKeyCode::Colon => "",
        VirtualKeyCode::Comma => "Comma",
        VirtualKeyCode::Convert => "",
        VirtualKeyCode::NumpadDecimal => "NumpadDecimal",
        VirtualKeyCode::NumpadDivide => "NumpadDivide",
        VirtualKeyCode::Equals => "Equal",
        VirtualKeyCode::Grave => "Backquote",
        VirtualKeyCode::Kana => "",
        VirtualKeyCode::Kanji => "",
        VirtualKeyCode::LAlt => "AltLeft",
        VirtualKeyCode::LBracket => "BracketLeft",
        VirtualKeyCode::LControl => "ControlLeft",
        VirtualKeyCode::LShift => "ShiftLeft",
        VirtualKeyCode::LWin => "",
        VirtualKeyCode::Mail => "",
        VirtualKeyCode::MediaSelect => "",
        VirtualKeyCode::MediaStop => "",
        VirtualKeyCode::Minus => "Minus",
        VirtualKeyCode::NumpadMultiply => "NumpadMultiply",
        VirtualKeyCode::Mute => "",
        VirtualKeyCode::MyComputer => "",
        VirtualKeyCode::NavigateForward => "",
        VirtualKeyCode::NavigateBackward => "",
        VirtualKeyCode::NextTrack => "",
        VirtualKeyCode::NoConvert => "",
        VirtualKeyCode::NumpadComma => "NumpadComma",
        VirtualKeyCode::NumpadEnter => "NumpadEnter",
        VirtualKeyCode::NumpadEquals => "NumpadEqual",
        VirtualKeyCode::OEM102 => "",
        VirtualKeyCode::Period => "Period",
        VirtualKeyCode::PlayPause => "",
        VirtualKeyCode::Power => "",
        VirtualKeyCode::Plus => "Plus",
        VirtualKeyCode::PrevTrack => "",
        VirtualKeyCode::RAlt => "AltRight",
        VirtualKeyCode::RBracket => "BracketRight",
        VirtualKeyCode::RControl => "ControlRight",
        VirtualKeyCode::RShift => "ShiftRight",
        VirtualKeyCode::RWin => "",
        VirtualKeyCode::Semicolon => "Semicolon",
        VirtualKeyCode::Slash => "Slash",
        VirtualKeyCode::Sleep => "",
        VirtualKeyCode::Stop => "",
        VirtualKeyCode::NumpadSubtract => "NumpadSubtract",
        VirtualKeyCode::Sysrq => "",
        VirtualKeyCode::Tab => "Tab",
        VirtualKeyCode::Underline => "",
        VirtualKeyCode::Unlabeled => "",
        VirtualKeyCode::VolumeDown => "",
        VirtualKeyCode::VolumeUp => "",
        VirtualKeyCode::Wake => "",
        VirtualKeyCode::WebBack => "",
        VirtualKeyCode::WebFavorites => "",
        VirtualKeyCode::WebForward => "",
        VirtualKeyCode::WebHome => "",
        VirtualKeyCode::WebRefresh => "",
        VirtualKeyCode::WebSearch => "",
        VirtualKeyCode::WebStop => "",
        VirtualKeyCode::Yen => "",
        VirtualKeyCode::Caret => "Caret",
        VirtualKeyCode::Copy => "Copy",
        VirtualKeyCode::Paste => "Paste",
        VirtualKeyCode::Cut => "Cut",
    }
}
