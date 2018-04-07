use gtk;
use gtk::prelude::*;

use views::trending;
use NAME;

pub fn get_headerbar(
    stack: &gtk::Stack,
    revealer: &gtk::Revealer,
    viewport: &gtk::Viewport,
) -> gtk::HeaderBar {
    let builder = gtk::Builder::new_from_resource("/com/github/kil0meters/sentinel/gtk/headerbar.ui");

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
        trending::refresh(&viewport);
    }));

    stack_switcher.set_stack(stack);

    headerbar.set_title(NAME);
    headerbar
}
