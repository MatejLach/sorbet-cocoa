#![feature(specialization)]

// Ported from:
// https://gist.github.com/lucamarrocco/2b06c92e4e6df01de04b

#[macro_use] extern crate objc;
#[macro_use] extern crate sorbet_cocoa as cocoa;

use cocoa::{Duck, Id, ShareId,
            IsNSApplication, IsNSWindowController, IsNSWindow, IsNSMenu};

struct AppDelegate {
    super_: Id<cocoa::NSObject>,
    app: ShareId<cocoa::NSApplication>,
    controller: ShareId<cocoa::NSWindowController>
}

impl AppDelegate {
    fn new(app: ShareId<cocoa::NSApplication>) -> Self {
        let controller = NiblessWindowController::new();
        let controller: Id<cocoa::NSWindowController> = controller.duck();

        AppDelegate {
            super_: cocoa::NSObject::new(),
            app: app,
            controller: controller.share()
        }
    }
}

objc_inherit! {
    impl Object for AppDelegate {
        type Super = NSObject;

        let super_ = self.super_;
    }
}

impl cocoa::IsNSApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&self, _: ShareId<cocoa::NSNotification>) {
        println!("application started!");
        self.controller.show_window(None);
        self.app.activate_ignoring_other_apps(true);
    }

    fn application_will_terminate(&self, _: ShareId<cocoa::NSNotification>) {
        println!("application terminated!");
    }

    fn application_should_terminate_after_last_window_closed(&self, _: ShareId<cocoa::NSApplication>) -> bool {
        true
    }
}

fn set_main_menu(app: &cocoa::NSApplication) {
    let tree = vec![
        ("Apple", vec![
            cocoa::NSMenuItem::new("Quit", Some(sel!(terminate:)), "q")
        ])
    ];

    let main_menu = cocoa::NSMenu::new("MainMenu");
    for (title, items) in tree {
        let submenu = cocoa::NSMenu::new(title).share();
        if let Some(item) = main_menu.add_item_with_title_action_key_equivalent(title, None, "") {
            main_menu.set_submenu_for_item(Some(submenu.clone()), item);
            for item in items {
                submenu.add_item(item.share())
            }
        }
    }

    app.set_main_menu(Some(main_menu.share()));
}

struct NiblessWindowController {
    super_: Id<cocoa::NSWindowController>
}

objc_inherit! {
    impl Object for NiblessWindowController {
        type Super = NSWindowController;

        let super_ = self.super_;
    }
}

impl NiblessWindowController {
    fn new() -> Self {
        let rect = cocoa::ns_make_rect(0.0, 0.0, 480.0, 320.0);
        let style = (1 << 0) | (1 << 1) | (1 << 3); // cocoa::NSTitledWindowMask | cocoa::NSClosableWindowMask | cocoa::NSResizableWindowMask;
        let backing  = cocoa::NSBackingStoreType::Buffered;
        let window = cocoa::NSWindow::new(rect, style, backing, false);
        window.set_title("App");

        let super_ = cocoa::NSWindowController::new(Some(window.share()));

        NiblessWindowController {
            super_: super_
        }
    }
}

fn main() {
    let app = cocoa::NSApplication::shared_application();
    let delegate = AppDelegate::new(app.clone()).duck();
    unsafe { app.set_delegate(Some(delegate)); }

    set_main_menu(&app);

    app.set_activation_policy(cocoa::NSApplicationActivationPolicy::Regular);
    app.run();
}
