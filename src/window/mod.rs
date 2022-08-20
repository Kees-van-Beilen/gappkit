mod macos;
mod windowDescriptor;
pub use windowDescriptor::WindowDescriptor;
use crate::ui::{ContentView,View};

pub struct Window{
    ///Do not modify
    pub internal_title:String,

    #[cfg(target_os="macos")]
    macos_window_handle: cocoa::base::id
}



impl Window{
    pub fn new<Descriptor>(t_descriptor:Descriptor)->Self where Descriptor:Into<WindowDescriptor>{
        let descriptor:WindowDescriptor = t_descriptor.into();
        #[cfg(target_os="macos")]
        unsafe{
            use cocoa::base::{YES,NO,nil};
            use cocoa::appkit::{NSWindow,NSWindowStyleMask,NSBackingStoreBuffered};
            use cocoa::foundation::{NSRect,NSPoint,NSSize,NSString};
            
            let rect = NSRect::new(NSPoint::new(0.0, 0.0),NSSize::new(500.0, 400.0));
            let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(rect, NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSResizableWindowMask, NSBackingStoreBuffered, NO);
            window.center();
            window.setTitle_(NSString::alloc(nil).init_str(descriptor.title.as_str()));
            window.makeKeyAndOrderFront_(nil);
            return Self { internal_title: descriptor.title, macos_window_handle: window };
        }
        #[cfg(target_os="windows")]
        unsafe{
            panic!("not available on windows");
        }
        
        
    }

    //set window ui
    pub fn ui(&self,view:View){
        use cocoa::appkit::{NSWindow,NSView,NSLayoutConstraint};
        use cocoa::base::{id,nil,NO,YES};
        use cocoa::foundation::NSArray;
        use objc::*;
        use macos::*;

        let children = view.get_children().unwrap();
        let sibling_count:i32 = children.len() as i32;
        
        if sibling_count > 1 {panic!("A root view can only contain 1 child")}
        for child in children.iter() {
            let t:id = child.build(Box::new(&view), sibling_count);
            unsafe{
                let master:id = NSWindow::contentView(self.macos_window_handle);
                NSView::addSubview_(master, t);
                if child.is_lazy() {
                    let x:id = msg_send![class!(NSLayoutConstraint), constraintWithItem: t attribute:(NSLayoutAttribute::CenterX as i64) relatedBy:(NSLayoutRelation::Equal as i64) toItem: master attribute: (NSLayoutAttribute::CenterX as i64) multiplier:1.0 constant:0.0];
                    let y:id = msg_send![class!(NSLayoutConstraint), constraintWithItem: t attribute:(NSLayoutAttribute::CenterY as i64) relatedBy:(NSLayoutRelation::Equal as i64) toItem: master attribute: (NSLayoutAttribute::CenterY as i64) multiplier:1.0 constant:0.0];
                    //makes the window not panic;
                    let _:() = msg_send![t, setTranslatesAutoresizingMaskIntoConstraints:NO];
                    //makes the window not panic;
                    let _:() = msg_send![master, setTranslatesAutoresizingMaskIntoConstraints:YES];
                    NSLayoutConstraint::activateConstraints(nil, NSArray::arrayWithObjects(nil, &[x,y]));

                } else {
                    //maximise the child
                    let lead:id = msg_send![class!(NSLayoutConstraint), constraintWithItem: t attribute:(NSLayoutAttribute::Width as i64) relatedBy:(NSLayoutRelation::Equal as i64) toItem: master attribute: (NSLayoutAttribute::Width as i64) multiplier:1.0 constant:0.0];
                    let trail:id = msg_send![class!(NSLayoutConstraint), constraintWithItem: t attribute:(NSLayoutAttribute::Height as i64) relatedBy:(NSLayoutRelation::Equal as i64) toItem: master attribute: (NSLayoutAttribute::Height as i64) multiplier:1.0 constant:0.0];
                    //makes the window not panic;
                    let _:() = msg_send![t, setTranslatesAutoresizingMaskIntoConstraints:NO];
                    //makes the window not panic;
                    let _:() = msg_send![master, setTranslatesAutoresizingMaskIntoConstraints:YES];

                    NSLayoutConstraint::activateConstraints(nil, NSArray::arrayWithObjects(nil, &[lead,trail]));
                }

            }
           

        }
    }
}