// extern crate gappkit;
use gappkit::prelude::*;

fn main(){
    App::new("hello world")
        .on_launch(launch)
        .run()
}
fn launch(_app:&App){
    Window::new(WindowDescriptor{
        title:"title".to_string()
    }).ui(
        View{
            children:vec![
                Box::new(
                    HStackView::new(vec![
                        Box::new(ImageView::new(format!("{}/resources/paris.jpeg",std::env::current_dir().unwrap().to_str().unwrap_or("./")).as_str())),
                        Box::new(ImageView::new(format!("{}/resources/london.jpeg",std::env::current_dir().unwrap().to_str().unwrap_or("./")).as_str())),
                        Box::new(TextView::new("Hello world".to_string()))
                    ])
                )
            ],
            ..Default::default()
        }
    );
}