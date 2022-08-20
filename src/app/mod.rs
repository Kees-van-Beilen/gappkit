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
    pub state:AppState
}

static mut EventQueue:Vec<(String,fn(&App))> = vec![];
static mut LifeTimeApp:*mut App = std::ptr::null_mut();

use objc::runtime::Object;
use objc::runtime::Sel;
use cocoa::base::id;

extern fn macos_app_did_finish_launching(this: &Object, _cmd: Sel, _notification: id) {
    unsafe{
        for (event,then) in EventQueue.iter() {
            if event == "onLaunch" {
                then(&*LifeTimeApp);
            }
        }
    }
}

impl App {

    pub fn new<ConfigType>(config:ConfigType)->&'static App where ConfigType: Into<AppConfig>{
        let app =  App {
            config: config.into(),
            state: AppState::default()
        };
        unsafe{
            LifeTimeApp = Box::into_raw(Box::new(app));
            return &*LifeTimeApp;
        }
       
    }

    ///Called when on
    ///macos/ios: applicationDidFinishLaunching event was fired
    ///windows: 
    pub fn on_launch(&self,then:fn(&App))->&Self{
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
                (applicationWillFinishLaunching:) => macos_app_did_finish_launching as extern fn(&Object, Sel, id)
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

