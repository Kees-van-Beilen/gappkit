use crate::ui::{ContentView,InstanceSize,Size};
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
        None
    }
    fn get_children_mut(&mut self)->Option<&mut Vec<Box<dyn ContentView>>> {
        None
    }
    
    #[cfg(target_os="macos")]
    fn build(&mut self,sibling_count:i32)->cocoa::base::id {
        use cocoa::foundation::{NSString};
        use cocoa::base::{nil,id};
        use objc::*;
        unsafe{
            let content_str:id = NSString::alloc(nil).init_str(self.content.as_str());
            let textfield:id = msg_send![class!(NSTextField), labelWithString:content_str];
            return textfield;

        }
        return nil;
    }
}