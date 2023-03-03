use crate::{AppEvent, Ecs};
use gw_util::value::Value;
use std::fmt::Debug;

/// The result from a call to one of the [`Screen`] event functions
pub enum ScreenResult {
    /// Continue to process the frame
    Continue,
    /// Push a new screen onto the stack
    Push(Box<dyn Screen>),
    /// Replace the current screen with this new one
    Replace(Box<dyn Screen>),
    /// Pop the current screen off the stack
    Pop,
    /// Quit the application
    Quit,
    /// Save a screen capture to the given filename
    Capture(String),
}

impl Debug for ScreenResult {
    /// Shows the name of the enum value
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScreenResult::Continue => "Continue".to_owned(),
                ScreenResult::Push(_) => "Push".to_owned(),
                ScreenResult::Replace(_) => "Replace".to_owned(),
                ScreenResult::Pop => "Pop".to_owned(),
                ScreenResult::Quit => "Quit".to_owned(),
                ScreenResult::Capture(name) => format!("Capture({})", name),
            }
        )
    }
}

impl PartialEq for ScreenResult {
    /// Compares the enum value, but not any Screen
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScreenResult::Continue, ScreenResult::Continue) => true,
            (ScreenResult::Push(_), ScreenResult::Push(_)) => true,
            (ScreenResult::Replace(_), ScreenResult::Replace(_)) => true,
            (ScreenResult::Pop, ScreenResult::Pop) => true,
            (ScreenResult::Quit, ScreenResult::Quit) => true,
            (ScreenResult::Capture(a), ScreenResult::Capture(b)) => a == b,
            (_, _) => false,
        }
    }
}

pub type BoxedScreen = Box<dyn Screen>;

/// A screen to handle input, update, and render events
#[allow(unused_variables)]
pub trait Screen {
    /// Called once, when the screen is first added to the [`crate::Runner`]
    fn setup(&mut self, ecs: &mut Ecs) {
        self.resize(ecs);
    }

    /// Called when the app is resized
    fn resize(&mut self, ecs: &mut Ecs) {}

    /// Returns whether or not this screen is full size, if not the [`crate::Runner`] will render the screens below.
    fn is_full_screen(&self) -> bool {
        true
    }

    /// Called when another screen is pushed on top of this one
    fn pause(&mut self, ecs: &mut Ecs) {}

    /// Called when this screen becomes the topmost screen
    fn resume(&mut self, ecs: &mut Ecs) {}

    /// called when a message is sent via app.send_message(...)
    fn message(&mut self, ecs: &mut Ecs, id: String, data: Option<Value>) -> ScreenResult {
        ScreenResult::Continue
    }

    /// Called once for each input event that occurred in this frame
    fn input(&mut self, ecs: &mut Ecs, event: &AppEvent) -> ScreenResult {
        ScreenResult::Continue
    }

    /// Called at the goal fps, can be called multiple times per frame
    fn update(&mut self, ecs: &mut Ecs) -> ScreenResult {
        ScreenResult::Continue
    }

    /// Called at the goal fps, can be called multiple times per frame
    fn background_update(&mut self, ecs: &mut Ecs) {}

    /// Called once at the end of the frame
    fn render(&mut self, ecs: &mut Ecs) {}

    /// Called when this screen is popped from the stack
    fn teardown(&mut self, ecs: &mut Ecs) {}
}
