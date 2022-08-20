// extern crate gappkit;
use gappkit::prelude::*;

fn main(){
    App::new("hello world")
    .on_launch(launch)
    .run()
}
fn launch(app:&App){
    println!("application did finish launching");
    // Window::new(())
    // println!("{}",app.config.name);

    Window::new(WindowDescriptor{
        title:"title".to_string()
    }).ui(
        View{
            children:vec![
                Box::new(ImageView::new("/Users/kbeilen/Desktop/homework/coding/cargoProjects/gappkit/resources/paris.jpeg"))
            ],
            ..Default::default()
        }
    );
        // .ui(View!{
        //     HStack!{
        //         Text!("Hello world")
        //         Text!("Hello world")
        //     }
        // })
}