pub struct AppState{
    pub didLaunch:bool
}
impl Default for AppState{
    fn default() -> Self {
        Self {
          didLaunch:false
        }
    }
}
pub struct App{
    pub config:AppConfig,
    pub state:AppState,
    pub windows:Vec<crate::window::Window>
}

static mut EventQueue:Vec<(String,fn(&mut App))> = vec![];
pub static mut LifeTimeApp:*mut App = std::ptr::null_mut();

use std::ffi::CString;
// use core::ffi::CStr;
use std::os::raw::c_void;

use objc::class;
use objc::runtime::Object;
use objc::runtime::Sel;
use cocoa::base::id;
use objc::runtime::ivar_getTypeEncoding;

use crate::ui::ButtonView;

extern fn macos_app_did_finish_launching(this: &Object, _cmd: Sel, _notification: id) {
    unsafe{
        for (event,then) in EventQueue.iter() {
            if event == "onLaunch" {
                then(&mut *LifeTimeApp);
            }
        }
    }
}
extern fn macos_click_event(this: &Object, _cmd: Sel, _notification: id) {
    
    unsafe{
        // _notification
        // let cls = objc::runtime::object_getClass(_n/otification);
        // let cls_name_c = CString::from_raw(objc::runtime::class_getName(cls) as *mut i8);
        // let cls_name = cls_name_c.to_str().unwrap_or_default();

        let obj: &*const c_void = Object::get_ivar(&*_notification, "_button_view_struct");
        let o  = ((*obj) as *const ButtonView);
        println!("a button was clicked titled: {}",(*o).title);
        if (*o).callback.is_some() {
            (*o).callback.unwrap()(&*o);
        }
    }
}

impl App {

    pub fn new<ConfigType>(config:ConfigType)->&'static App where ConfigType: Into<AppConfig>{
        let app =  App {
            config: config.into(),
            state: AppState::default(),
            windows: vec![]
        };
        unsafe{
            //create GAPPButton class
            use objc::declare::ClassDecl;
            use objc::class;

            let mut decl = ClassDecl::new("GAPPButton", class!(NSButton)).unwrap();
            decl.add_ivar::<*const c_void>("_button_view_struct");
            decl.register();
            LifeTimeApp = Box::into_raw(Box::new(app));
            return &*LifeTimeApp;
        }
       
    }

    ///Called when on
    ///macos/ios: applicationDidFinishLaunching event was fired
    ///windows: 
    pub fn on_launch(&self,then:fn(&mut App))->&Self{
        // then(self);
        unsafe{
            EventQueue.push(("onLaunch".to_string(),then));
        }
        self
    }

    

    pub fn run(&self){
        //macos
        #[cfg(target_os="macos")]
        unsafe{
            
            use cocoa::appkit::{NSApp,NSApplication,NSApplicationActivationPolicyRegular,NSMenu,NSMenuItem,NSWindow,NSWindowStyleMask,NSBackingStoreBuffered};
            use cocoa::base::{YES,NO,nil};
            use cocoa::foundation::{NSString,NSRect,NSPoint,NSSize,NSAutoreleasePool};
            use cocoa::delegate;
            // use cocoa::base::{SEL,Class};
            use objc::{sel,msg_send,sel_impl,class};
            //register the application delegate
            let nsapp = NSApp();
            nsapp.setDelegate_(delegate!("GAPPApplicationDelegate",{
                (applicationWillFinishLaunching:) => macos_app_did_finish_launching as extern fn(&Object, Sel, id),
                (click_event:) => macos_click_event as extern fn(&Object, Sel, id)
            }));
            let menubar = NSMenu::new(nil);
            let app_menu = NSMenu::new(nil);
            let app_menu_item = NSMenuItem::new(nil);
            // let app_name = NSString::alloc(nil).init_str(self.config.name);
            let quit = NSString::alloc(nil).init_str((format!("{}{}","Quit ",self.config.name).as_str()));
            let quit_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(quit, cocoa::base::selector("terminate:"), NSString::alloc(nil).init_str("q"));
            NSApplication::setMainMenu_(nsapp, menubar);
            NSMenu::addItem_(app_menu, quit_menu_item);
            NSMenuItem::setSubmenu_(app_menu_item, app_menu);
            NSMenu::addItem_(menubar, app_menu_item);
            NSApplication::setActivationPolicy_(nsapp, NSApplicationActivationPolicyRegular);
            //yust a test
            // NSApplication::setMainMenu_(nsapp, menu);
            NSApplication::activateIgnoringOtherApps_(nsapp, YES);

            //transfer
            // unsafe{
            //     LifeTimeApp = std::ptr::addr_of_mut!(*self);
            // }
            NSApplication::run(nsapp);
        }
        #[cfg(target_os="windows")]
        {
            panic!("Windows has not been implemented")
        }
    }

}
pub struct AppConfig{
    pub name:&'static str
}
impl Default for AppConfig{
    fn default() -> Self {
        Self {
            name:"GappKit app"
        }
    }
}
impl From<&'static str> for AppConfig{
    fn from(name: &'static str) -> Self {
        return Self {name:name, ..Default::default()};
    }
}
impl From<()> for AppConfig{
    fn from(_: ()) -> Self {
        return Self::default();
    }
}

