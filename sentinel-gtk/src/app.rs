use gtk;
use gdk;
use gio;
use gtk::prelude::*;
use gio::prelude::*;

use std::cell::RefCell;
use std::sync::mpsc::Receiver;

use {NAME, TAGLINE};

use static_resources;
use preferences;
use headerbar;
use sentinel_api::youtube;
use views::trending;

// Define thread local storage keys.
thread_local! {
    pub static VPLAYER: RefCell<Option<(gtk:: Stack, gtk::Overlay)>> = RefCell::new(None);
    #[allow(unknown_lints, type_complexity)]
    pub static TRENDING: RefCell<Option<(
        gtk::Viewport,
        Receiver<Option<Vec<youtube::Video>>>,
    )>> = RefCell::new(None);
    #[allow(unknown_lints, type_complexity)]
    pub static THUMBNAILS: RefCell<Option<(
        Vec<gtk::Image>,
        Receiver<Option<String>>,
    )>> = RefCell::new(None);
}

pub fn run_app() -> Result<(), String> {
    let application = match gtk::Application::new(
        Some("com.github.kil0meters.sentinel"),
        gio::ApplicationFlags::empty(),
    ) {
        Ok(app) => {
            app.connect_activate(move |app| { build_ui(app); });
            app
        }
        Err(e) => {
            return Err(format!("Failed to create user interface: {:?}", e));
        }
    };

    application.run(&[]);

    Ok(())
}

fn build_ui(app: &gtk::Application) {
    let builder = gtk::Builder::new_from_resource("/com/github/kil0meters/sentinel/gtk/interface.ui");

    let win = gtk::ApplicationWindow::new(app);
    win.set_default_size(732, 500);
    win.set_gravity(gdk::Gravity::Center);

    let vbox: gtk::Box = builder.get_object("vbox").unwrap();
    let revealer: gtk::Revealer = builder.get_object("search_revealer").unwrap();
    let vplayer_stack: gtk::Stack = builder.get_object("vplayer_stack").unwrap();
    let vplayer_overlay: gtk::Overlay = builder.get_object("vplayer_overlay").unwrap();
    let stack: gtk::Stack = builder.get_object("stack").unwrap();
    let viewport: gtk::Viewport = builder.get_object("trending_viewport").unwrap();

    trending::refresh(&viewport);

    // move vplayer_stack and vplayer_overlay into a thread local storage
    // key to be used later
    VPLAYER.with(move |vplayer| {
        *vplayer.borrow_mut() = Some((vplayer_stack, vplayer_overlay));
    });

    let headerbar = headerbar::get_headerbar(&stack, &revealer, &viewport);

    win.add(&vbox);
    win.set_title(NAME);
    win.set_wmclass(NAME, NAME);
    win.set_titlebar(&headerbar);

    win.show_all();
    win.activate();
}
