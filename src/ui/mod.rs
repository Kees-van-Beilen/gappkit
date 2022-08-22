mod textview;
mod imageview;
mod hstack;
mod vstack;
mod button;
pub use textview::TextView;
pub use imageview::{ImageView,Image};
pub use hstack::HStackView;
pub use vstack::VStackView;
pub use button::ButtonView;
pub enum InstanceSize<T> {
    ///no value assigned
    None,
    ///value decided before ui has been build
    Some(T),
    ///the ui builder has to define it's size
    Instance
}
pub trait ContentView{
     fn is_lazy(&self)->bool;
     fn get_min_size(&self)->InstanceSize<Size<i32>>;
     fn get_max_size(&self)->InstanceSize<Size<i32>>;
     fn get_children(&self)->Option<&Vec<Box<dyn ContentView>>>;
     fn get_children_mut(&mut self)->Option<&mut Vec<Box<dyn ContentView>>>;
     #[cfg(target_os="macos")]
     fn build(&mut self,sibling_count:i32)->cocoa::base::id;
}

pub struct Size<T>{
    width:T,
    height:T
}

impl<T> Size<T> {
    #[inline(always)]
    pub fn new(width:T,height:T)->Self{
        Self {
            width,
            height
        }
    }
}


pub struct View{
    ///when true use the object's min/max dimensions
    /// when false try to fill the most available content
    pub is_lazy:bool,

    ///lazy min size (may be used when not lazy to determine size priority)
    pub min_size:InstanceSize<Size<i32>>,
    ///lazy max size
    pub max_size:InstanceSize<Size<i32>>,
    
    pub children:Vec<Box<dyn ContentView>>
}

impl Default for View {
    fn default() -> Self {
        Self {
            is_lazy: false,
            min_size: InstanceSize::None,
            max_size: InstanceSize::None,
            children: vec![]
        }
    }
}

impl ContentView for View{
    fn is_lazy(&self)->bool {
        false
    }

    fn get_min_size(&self)->InstanceSize<Size<i32>> {
        InstanceSize::None
    }

    fn get_max_size(&self)->InstanceSize<Size<i32>> {
        InstanceSize::None
    }

    fn get_children(&self)->Option<&Vec<Box<dyn ContentView>>> {
        Some(&self.children)
    }
    fn get_children_mut(&mut self)->Option<&mut Vec<Box<dyn ContentView>>> {
        Some(&mut self.children)
    }

    fn build(&mut self,sibling_count:i32)->cocoa::base::id {
        panic!("cannot build view")
    }
}


