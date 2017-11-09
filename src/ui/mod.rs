//  Copyright (C) 2017  Kil0meters
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
mod utils;
mod headerbar;
mod views;
mod widgets;
mod preferences;

use gtk;
use gdk;
use gio;
use gtk::prelude::*;
use gio::prelude::*;

use std::cell::RefCell;
use std::sync::mpsc::Receiver;

use {NAME, TAGLINE};

use lib::youtube;
use ui::views::trending;

// Define thread local storage keys.
thread_local! {
    static VPLAYER: RefCell<Option<(gtk:: Stack, gtk::Overlay)>> = RefCell::new(None);
    #[allow(unknown_lints, type_complexity)]
    static TRENDING: RefCell<Option<(
        gtk::Viewport,
        Receiver<Option<Vec<youtube::Video>>>,
    )>> = RefCell::new(None);
    #[allow(unknown_lints, type_complexity)]
    static THUMBNAILS: RefCell<Option<(
        Vec<gtk::Image>,
        Receiver<Option<String>>,
    )>> = RefCell::new(None);
}

pub fn run_app() -> Result<(), String> {
    let application = match gtk::Application::new(
        Some("com.github.kil0meters.youtube-client"),
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
    let builder = include_str!("../../data/ui/interface.ui");
    let builder = gtk::Builder::new_from_string(builder);

    let win = gtk::ApplicationWindow::new(app);
    win.set_default_size(732, 500);
    win.set_gravity(gdk::Gravity::Center);

    let app_menu: gio::Menu = builder.get_object("app_menu").unwrap();

    let preferences = gio::SimpleAction::new("preferences", None);
    let about = gio::SimpleAction::new("about", None);
    let quit = gio::SimpleAction::new("quit", None);

    preferences::initialize(&preferences, &win);

    let about_dialog = gtk::AboutDialog::new();
    about_dialog.set_program_name(NAME);
    about_dialog.set_authors(&["Kil0meters <kil0meters@protonmail.com>"]);
    about_dialog.set_comments(TAGLINE);
    about_dialog.set_copyright("© Kil0meters 2017");
    about_dialog.set_license_type(gtk::License::Gpl30);
    about_dialog.set_transient_for(&win);
    about_dialog.set_wmclass(NAME, NAME);
    about_dialog.connect_response(move |dialog, _| { dialog.hide(); });

    about.connect_activate(move |_, _| { about_dialog.run(); });
    quit.connect_activate(clone!(win => move |_, _| {
        win.destroy();
    }));
    app.add_action(&preferences);
    app.add_action(&about);
    app.add_action(&quit);
    app.set_app_menu(&app_menu);

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
