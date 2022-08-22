use crate::ui::{ContentView,InstanceSize,Size};
pub struct HStackView{
    pub children:Vec<Box<dyn ContentView>>
}

impl HStackView{
    pub fn new(children:Vec<Box<dyn ContentView>>)->Self{
        Self {
            children
        }
    }
}
impl ContentView for HStackView{
    fn is_lazy(&self)->bool {
        false
    }

    fn get_min_size(&self)->InstanceSize<Size<i32>> {
        //The size has to be decided by ui instance method
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
    #[cfg(target_os="macos")]
    fn build(&mut self,sibling_count:i32)->cocoa::base::id {
        use cocoa::foundation::{NSString,NSArray};
        use cocoa::base::{nil,id};
        use objc::*;
        unsafe{
            // let content_str:id = NSString::alloc(nil).init_str(self.content.as_str());
            // let textfield:id = msg_send![class!(NSStackView), stackViewWithViews:];
            // return textfield;
            let child_count = self.children.len();
            let mut children_array:Vec<id> = vec![];

            for child in self.children.iter_mut() {
                let child_view = child.build( child_count as i32);
                if child.is_lazy() {

                }else{

                }
                children_array.push(child_view);
            }
            let views  = NSArray::arrayWithObjects(nil, &children_array);
            let root:id = msg_send![class!(NSStackView), stackViewWithViews:views];
            let _:() = msg_send![root, setDistribution:1];
            let _:() = msg_send![root, setSpacing:0.0];
            return root;
        }
        return nil;
    }
}