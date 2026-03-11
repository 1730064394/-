pub mod window;
pub mod button;
pub mod label;
pub mod input;
pub mod events;
pub mod engine;

pub use window::Window;
pub use button::Button;
pub use label::Label;
pub use input::Input;
pub use events::{Event, EventType, EventHandler};
pub use engine::{GuiEngine, init_gui_engine, get_gui_engine};

pub trait Control: events::EventHandler {
    fn get_id(&self) -> &str;
    fn draw(&self);
    fn set_position(&mut self, x: f32, y: f32);
    fn set_size(&mut self, width: f32, height: f32);
    fn set_visible(&mut self, visible: bool);
    fn is_visible(&self) -> bool;
}

#[cfg(test)]
mod tests;
