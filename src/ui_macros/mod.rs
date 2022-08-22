use crate::ui::{View};
#[macro_export]
macro_rules! View {
    ( $( $x:expr )* ) => {
        {
            let mut view = gappkit::ui::View{
                ..Default::default()
            };
            $(
                view.children.push($x);
            )*
            view
        }
    };
}
#[macro_export]
macro_rules! Text {
    ( $x:expr ) => {
        {
            let mut text_view = gappkit::ui::TextView::new($x);
            Box::new(text_view)
        }
    };
}
#[macro_export]
macro_rules! HStack {
    ( $( $x:expr )* ) => {
        {
            let mut view = gappkit::ui::HStackView{
                children:vec![$(
                    ($x),
                )*]
            };
            
            Box::new(view)
        }
    };
}
#[macro_export]
macro_rules! VStack {
    ( $( $x:expr )* ) => {
        {
            let mut view = gappkit::ui::VStackView{
                children:vec![$(
                    ($x),
                )*]
            };
            Box::new(view)
        }
    };
}
#[macro_export]
macro_rules! Image {
    ( $x:expr) => {
        {
            let mut image_view = gappkit::ui::ImageView::new($x);
            Box::new(image_view)
        }
    };
}
#[macro_export]
macro_rules! Button {
    ( $x:expr) => {
        {
            let mut button_view = gappkit::ui::ButtonView::new($x);
            Box::new(button_view)
        }
    };
}

#[macro_export]
macro_rules! _ScaleAspect {
    ( $x:expr ) => {
        {
            let mut image_view = gappkit::ui::ImageView::new($x);
            Box::new(image_view)
        }
    };
}