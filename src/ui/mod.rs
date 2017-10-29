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

mod headerbar;
mod utils;
mod widgets;

use gtk;
use gio;

use gtk::prelude::*;
use gio::prelude::*;

use {NAME, TAGLINE};

// http://gtk-rs.org/tuto/closures
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


pub fn run_app() -> Result<(), String> {
    let application = match gtk::Application::new(
        Some("com.github.kil0meters.youtube-client"),
        gio::ApplicationFlags::empty(),
    ) {
        Ok(app) => {
            app.connect_activate(move |app| {
                build_ui(app);
            });
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
    let builder = gtk::Builder::new_from_string(include_str!("../../data/ui/interface.ui"));

    let win = gtk::ApplicationWindow::new(app);
    win.set_default_size(720, 500);

    let app_menu: gio::Menu = builder.get_object("app_menu").unwrap();

    let preferences = gio::SimpleAction::new("preferences", None);
    let about = gio::SimpleAction::new("about", None);
    let quit = gio::SimpleAction::new("quit", None);
    preferences.connect_activate(move |_, _| {
        println!("Filler");
    });


    about.connect_activate(clone!(win => move |_, _| {
        let about_dialog = gtk::AboutDialog::new();
        about_dialog.set_program_name(NAME);
        about_dialog.set_authors(&["Kil0meters <kil0meters@protonmail.com>"]);
        about_dialog.set_comments(TAGLINE);
        about_dialog.set_copyright("© Kil0meters 2017");
        about_dialog.set_license_type(gtk::License::Gpl30);
        about_dialog.set_transient_for(&win);
        about_dialog.set_wmclass(NAME, NAME);

        about_dialog.connect_response(move |dialog, _| {
            dialog.destroy();
        });

        about_dialog.run();
    }));
    quit.connect_activate(clone!(win => move |_, _| {
        win.destroy();
    }));
    app.add_action(&preferences);
    app.add_action(&about);
    app.add_action(&quit);
    app.set_app_menu(&app_menu);

    let vbox: gtk::Box = builder.get_object("vbox").unwrap();
    let revealer: gtk::Revealer = builder.get_object("search_revealer").unwrap();
    let stack: gtk::Stack = builder.get_object("stack").unwrap();
    let viewport: gtk::Viewport = builder.get_object("trending_viewport").unwrap();

    utils::refresh_trending(&viewport);

    let headerbar = headerbar::get_headerbar(&stack, &revealer, &viewport);

    win.add(&vbox);
    win.set_title(NAME);
    win.set_wmclass(NAME, NAME);
    win.set_titlebar(&headerbar);

    win.show_all();
    win.activate();
}
