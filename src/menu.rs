extern crate gtk;

use gtk::Inhibit;
use gtk::traits::*;

pub fn create_menu<F: Fn() + 'static>(quit_action: F) -> gtk::MenuBar {
    let bar = gtk::MenuBar::new();
    let file = gtk::MenuItem::new_with_label("File");
    let f_menu = gtk::Menu::new();
    let f_new = gtk::MenuItem::new_with_label("Launch new executable");
    let f_quit = gtk::MenuItem::new_with_label("Quit");
    f_quit.connect_activate(move |_| {
        quit_action();
        Inhibit(false);
    });
    let help = gtk::MenuItem::new_with_label("Help");
    let h_menu = gtk::Menu::new();
    let h_about = gtk::MenuItem::new_with_label("About");

    f_menu.append(&f_new);
    f_menu.append(&f_quit);
    file.set_submenu(Some(&f_menu));
    bar.append(&file);

    h_menu.append(&h_about);
    help.set_submenu(Some(&h_menu));
    bar.append(&help);
    bar
}
