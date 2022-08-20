#[macro_use]
// use cocoa::delegate;
pub mod app;
pub mod window;
pub mod ui;
pub mod prelude{
    pub use crate::app::*;
    pub use crate::window::*;
    pub use crate::ui::*;
}