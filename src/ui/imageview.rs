use cocoa::foundation::NSString;

use crate::ui::{ContentView,InstanceSize,Size};
pub struct Image{
    valid:bool,
    file:String,
    macos_image_handle:cocoa::base::id
}
impl Image {
    fn from_file(path:&str)->Self{
        unsafe{
            use cocoa::base::{nil, id};
            use cocoa::foundation::{NSData};
            let image_handle:id = cocoa::appkit::NSImage::alloc(nil).initWithContentsOfFile_( cocoa::foundation::NSString::alloc(nil).init_str(path));
            return Self { 
                valid: image_handle != nil, 
                file: path.to_string(), 
                macos_image_handle: image_handle
            };
        }
    }
}
pub struct ImageView{
    image: Image
}
impl ImageView{
    pub fn new(contents_of_file:&str)->Self{
        Self {
            image:Image::from_file(contents_of_file)
        }
    }
}
impl ContentView for ImageView{
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
        //has no children
        None
    }
    #[cfg(target_os="macos")]
    fn build(&self,parent:Box<&dyn ContentView>,sibling_count:i32)->cocoa::base::id {
        use cocoa::foundation::{NSString,NSSize};
        use cocoa::base::{nil,id};
        use cocoa::appkit::NSImageView;
        use objc::*;
        unsafe{
            // let content_str:id = NSString::alloc(nil).init_str(self.content.as_str());
            let image = self.image.macos_image_handle;
            // let tmp0:id = msg_send![class!(NSImageView), alloc];

            let view:id = msg_send![class!(NSImageView), imageViewWithImage: image];
            let _:() = msg_send![view, setFrameSize:NSSize::new(100.0, 100.0)];
            let _:() = msg_send![view, setImageScaling: 1];
            return view;

        }
        return nil;
    }
}