// extern crate gappkit;
use gappkit::{prelude::*, ui::ButtonView};

fn main(){
    App::new("hello world")
        .on_launch(launch)
        .run()
}


static mut counter:i32 = 1;
fn click(btn:&mut ButtonView){
    println!("clicked");
    unsafe {
        counter+=1;
        println!("Hello world: {}",counter);
        btn.set_title(format!("Hello world: {}",counter).as_str());
    }
}
fn launch(app:&mut App){

    let window = Window::new("test window")
        .ui(View!{
            Button!("Hello world: 1".to_string())
                .on_click(click)
        });
    app.windows.push(window);
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