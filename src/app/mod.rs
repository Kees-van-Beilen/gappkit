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
impl App {

    pub fn new<ConfigType>(config:ConfigType)->App where ConfigType: Into<AppConfig>{
        return App {
            config: config.into(),
            state: AppState::default()
        };
    }

    ///Called when on
    ///macos/ios: applicationDidFinishLaunching event was fired
    ///windows: 
    pub fn on_launch(&self,then:fn(&App))->&Self{
        then(self);
        self
    }

    pub fn run(&self){
        //macos
        #[cfg(target_os="macos")]
        unsafe{
            
            use cocoa::appkit::{NSApp,NSApplication,NSApplicationActivationPolicyRegular,NSMenu,NSMenuItem,NSWindow,NSWindowStyleMask,NSBackingStoreBuffered};
            use cocoa::base::{YES,NO,nil};
            use cocoa::foundation::{NSString,NSRect,NSPoint,NSSize,NSAutoreleasePool};

            //register the application delegate
            let nsapp = NSApp();
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

