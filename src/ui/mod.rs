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
     #[cfg(target_os="macos")]
     fn build(&self,parent:Box<&dyn ContentView>,sibling_count:i32)->cocoa::base::id;
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

    fn build(&self,parent:Box<&dyn ContentView>,sibling_count:i32)->cocoa::base::id {
        panic!("cannot build view")
    }
}

pub struct TextView{
    content:String
}
impl TextView{
    pub fn new(with_str:String)->Self{
        Self {
            content:with_str
        }
    }
}
impl ContentView for TextView{
    fn is_lazy(&self)->bool {
        true
    }

    fn get_min_size(&self)->InstanceSize<Size<i32>> {
        //The size has to be decided by ui instance method
        InstanceSize::Instance
    }

    fn get_max_size(&self)->InstanceSize<Size<i32>> {
        InstanceSize::Instance
    }

    fn get_children(&self)->Option<&Vec<Box<dyn ContentView>>> {
        //has no children
        None
    }
    #[cfg(target_os="macos")]
    fn build(&self,parent:Box<&dyn ContentView>,sibling_count:i32)->cocoa::base::id {
        // use cocoa::appkit::NSTextField;
        use cocoa::foundation::{NSString};
        use cocoa::base::{nil,id};
        // #[macros_use]
        // #[macro_use]
        // extern crate objc;
        // use objc::{class,sel,msg_send};
        use objc::*;
        unsafe{
            let content_str = NSString::alloc(nil).init_str(self.content.as_str());
            let textfield:id = msg_send![class!(NSTextField), labelWithString:content_str];
            return textfield;

        }
        return nil;
        // NSTextField::alloc(nil).initWithFrame_(NSRect::new(NSPoint::new(0, 0), NSSize::new(width, height)))
    }
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
