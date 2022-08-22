#[macro_use]
// use cocoa::delegate;
pub mod app;
pub mod window;
pub mod ui;
pub mod ui_macros;
pub mod prelude{
    pub use crate::app::*;
    pub use crate::window::*;
    pub use crate::ui;

    //allot of macros to export
    pub use crate::View;
    pub use crate::Text;
    pub use crate::Image;
    pub use crate::VStack;
    pub use crate::HStack;
    pub use crate::Button;
    
}