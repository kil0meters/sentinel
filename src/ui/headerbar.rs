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
    overlay: &gtk::Overlay,
) -> gtk::HeaderBar {
    let builder = gtk::Builder::new_from_string(include_str!("../../data/ui/headerbar.ui"));

    let headerbar: gtk::HeaderBar = builder.get_object("headerbar").unwrap();
    let refresh_button: gtk::Button = builder.get_object("refresh_button").unwrap();
    let search_button: gtk::Button = builder.get_object("search_button").unwrap();
    let stack_switcher: gtk::StackSwitcher = builder.get_object("stack_switcher").unwrap();

    revealer.set_reveal_child(false);

    search_button.connect_clicked(clone!(revealer => move |_| {
        let state = revealer.get_reveal_child();

        if !state {
            revealer.set_reveal_child(true);
        } else {
            revealer.set_reveal_child(false);
        }

    }));
    refresh_button.connect_clicked(clone!(overlay => move |_| {
        utils::refresh_trending(&overlay);
    }));

    stack_switcher.set_stack(stack);

    headerbar
}
