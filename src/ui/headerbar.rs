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

use gtk;
use gtk::prelude::*;

use ui::utils;

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

pub fn get_headerbar(
    stack: &gtk::Stack,
    revealer: &gtk::Revealer,
    viewport: &gtk::Viewport,
) -> gtk::HeaderBar {
    let builder = gtk::Builder::new_from_string(include_str!("../../data/ui/headerbar.ui"));

    let headerbar: gtk::HeaderBar = builder.get_object("headerbar").unwrap();
    let refresh_button: gtk::Button = builder.get_object("refresh_button").unwrap();
    let search_button: gtk::Button = builder.get_object("search_button").unwrap();
    let stack_switcher: gtk::StackSwitcher = builder.get_object("stack_switcher").unwrap();

    search_button.connect_clicked(clone!(revealer => move |_| {
        let state = revealer.get_reveal_child();

        if !state {
            revealer.set_reveal_child(true);
        } else {
            revealer.set_reveal_child(false);
        }

    }));
    refresh_button.connect_clicked(clone!(viewport => move |_| {
        utils::refresh_trending(&viewport);
    }));

    stack_switcher.set_stack(stack);

    headerbar
}
