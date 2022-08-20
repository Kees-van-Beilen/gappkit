use cocoa::foundation::NSString;
use crate::ui::{ContentView,View};
// #[cfg(target_os="macos")]
// {
//     use cocoa::appkit::{NSApp,NSApplication,NSApplicationActivationPolicyRegular,NSMenu,NSMenuItem,NSWindow,NSWindowStyleMask,NSBackingStoreBuffered};
//     use& cocoa::base::{YES,NO,nil};
//     use cocoa::foundation::{NSString,NSRect,NSPoint,NSSize,NSAutoreleasePool};
// }
enum NSLayoutAttribute {
    CenterX=9,
    CenterY=10
}
enum NSLayoutRelation{
    Equal=0
}

pub struct Window{
    ///Do not modify
    pub internal_title:String,
    #[cfg(target_os="macos")]
    macos_window_handle: cocoa::base::id
}
pub struct WindowDescriptor{
    pub title:String
}
impl From<&str> for WindowDescriptor{
    fn from(name: &str) -> Self {
        return Self {title:name.to_string(), ..Default::default()};
    }
}
impl From<()> for WindowDescriptor{
    fn from(_: ()) -> Self {
        return Self::default();
    }
}
impl Default for WindowDescriptor{
    fn default()->Self{
        return WindowDescriptor{
            title:"window".to_string()
        }
    }
}

impl Window{
    pub fn new<Descriptor>(t_descriptor:Descriptor)->Self where Descriptor:Into<WindowDescriptor>{
        let descriptor:WindowDescriptor = t_descriptor.into();
        #[cfg(target_os="macos")]
        unsafe{
            use cocoa::base::{YES,NO,nil};
            use cocoa::appkit::{NSWindow,NSWindowStyleMask,NSBackingStoreBuffered};
            use cocoa::foundation::{NSRect,NSPoint,NSSize};
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
        let children = view.get_children().unwrap();
        let sibling_count = children.len() as i32;
        // let paren = Box::new(view);
        if sibling_count > 1 {panic!("A root view can only contain 1 child")}
        for child in children.iter() {
            let t = child.build(Box::new(&view), sibling_count);
            unsafe{
                let master = NSWindow::contentView(self.macos_window_handle);
                NSView::addSubview_(master, t);
                if child.is_lazy() {
                    let x:id = msg_send![class!(NSLayoutConstraint), constraintWithItem: t attribute:(NSLayoutAttribute::CenterX as i64) relatedBy:(NSLayoutRelation::Equal as i64) toItem: master attribute: (NSLayoutAttribute::CenterX as i64) multiplier:1.0 constant:0.0];
                    let y:id = msg_send![class!(NSLayoutConstraint), constraintWithItem: t attribute:(NSLayoutAttribute::CenterY as i64) relatedBy:(NSLayoutRelation::Equal as i64) toItem: master attribute: (NSLayoutAttribute::CenterY as i64) multiplier:1.0 constant:0.0];
                    let rect = NSView::frame(master);
                    println!("x:{},y:{},w:{},h:{}",rect.origin.x,rect.origin.y,rect.size.width,rect.size.height);
                    let _:() = msg_send![t, setTranslatesAutoresizingMaskIntoConstraints:NO];
                    let _:() = msg_send![master, setTranslatesAutoresizingMaskIntoConstraints:YES];
                    NSLayoutConstraint::activateConstraints(nil, NSArray::arrayWithObjects(nil, &[x,y]));
                    let _:() = msg_send![x, setPriority: (500-10) as f32];
                    let _:() = msg_send![y, setPriority: (500-10) as f32];

                } else {
                    //maximise the child
                }

            }
           

        }
    }
}