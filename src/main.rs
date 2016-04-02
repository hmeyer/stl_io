extern crate gtk;
extern crate xplicit;

use gtk::{Inhibit, WidgetSignals};
use gtk::traits::*;

use xplicit::xplicit_widget;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let xw = xplicit_widget::XplicitWidget::new();

    window.set_default_size(500, 500);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.add(&xw.drawing_area);
    window.show_all();

    gtk::main();
}
