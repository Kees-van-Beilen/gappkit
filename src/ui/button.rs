use std::{collections::HashMap, sync::Mutex};

use cocoa::base::id;

use crate::ui::{ContentView,InstanceSize,Size};


pub struct ButtonView{
    pub title:String,
    pub callback:Option<fn(&ButtonView)>,
    pub id:i32,
    pub macos_handle:Option<id>
}

impl Drop for ButtonView {
    fn drop(&mut self) {
        println!("Dropping Buttonview!");
    }
}

impl ButtonView{
    pub fn new(with_str:String)->Self{
        Self {
            title:with_str,
            callback:None,
            id:0,
            macos_handle:None
        }
    }
    pub fn set_id(&mut self, to:i32)->&Self{
        self.id = to;
        return self;
    }
    pub fn on_click(&mut self,then:fn(&ButtonView))->&Self{
        self.callback = Some(then);
        self
    }
}
impl ContentView for ButtonView{
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
        use std::os::raw::c_void;

        use cocoa::appkit::NSImage;
        use cocoa::foundation::{NSString};
        use cocoa::base::{nil,id};
        use objc::*;

        unsafe{
            let content_str:id = NSString::alloc(nil).init_str(self.title.as_str());
            let del = cocoa::appkit::NSApp().delegate();
            let button:id = msg_send![class!(GAPPButton), buttonWithTitle:content_str target:del action: sel!(click_event:)];
            // button.set_ivar("_button_view_struct", &self);
            //trust me i know what i'm doing rust
            // let pointe = Box::leak(Box::new(self));
            let pointe = self as *const ButtonView;
            println!("creating {:p} as {:p} from {:p} as {:p}",pointe,pointe as *const _ as *const c_void,pointe,& *pointe);
            println!("tile: {}",self.title);
            let cvoid = pointe as *const c_void;
            let o = cvoid as *const ButtonView;
            println!("tile: {}",(*o).title);

            objc::runtime::Object::set_ivar(&mut *button, "_button_view_struct", pointe as *const c_void);
            //add the callback to the evt handlers
            unsafe{
                // let evt = BUTTON_EVT_HANDLERS.lock().unwrap();
                // if evt.is_none() {

                //     BUTTON_EVT_HANDLERS = Mutex::new(Some(Box::leak(Box::new(HashMap::new()))));
                // }
                // if self.callback.is_some() {
                //     evt.as_mut().unwrap().insert(button, self.callback.unwrap());
                // }
            }
            return button;

        }
        return nil;
    }
}