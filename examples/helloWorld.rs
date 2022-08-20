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

    Window::new(()).ui(
        View{
            children:vec![
                Box::new(TextView::new("Hello world".to_string()))
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