// extern crate gappkit;
use gappkit::prelude::*;

fn main(){
    App::new("hello world")
        .on_launch(launch)
        .run()
}

// fn click(btn:&ButtonView){
//     println!("clicked")
// }
fn launch(app:&mut App){

    Window::new("test window")
        .ui(View!{
            VStack!{
                Image!(format!("{}/resources/paris.jpeg",std::env::current_dir().unwrap().to_str().unwrap_or("./")).as_str())
                    .scale_aspect()
                Text!("This is a public domain image".to_string())
            }
        });
    // let mut wind = Window::new(WindowDescriptor{
    //     title:"title".to_string()
    // });
    // let mut root = View{
    //     ..Default::default()
    // };
    // root.children.push({
    //     let mut btn = ButtonView::new("Hello world".to_string());
    //     btn.on_click(click);
    //     Box::new(btn)
    // });

    // let mut btn = ButtonView::new("Hello world".to_string());
    // btn.on_click(click);
    // wind.ui(
    //     View{
    //         children:vec![
    //             Box::new(
    //                 VStackView::new(vec![
    //                     Box::new(ImageView::new(format!("{}/resources/paris.jpeg",std::env::current_dir().unwrap().to_str().unwrap_or("./")).as_str())),
    //                     Box::new(
    //                         btn
    //                     ),
    //                     Box::new(ImageView::new(format!("{}/resources/london.jpeg",std::env::current_dir().unwrap().to_str().unwrap_or("./")).as_str())),
    //                 ])
    //             )
    //         ],
    //         ..Default::default()
    //     }
    // );
    // //keeps the window structs alive
    // app.windows.push(wind);
    
}