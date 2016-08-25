
extern crate gtk;
extern crate gdk;
extern crate xplicit;

use gtk::Inhibit;
use gtk::traits::*;
use xplicit::xplicit_widget;
use xplicit::menu;

const FILENAME: &'static str = "xplicit.scad";


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_default_size(640, 480);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let debug_view = gtk::TextView::new();
    debug_view.set_wrap_mode(gtk::WrapMode::WordChar);
    let xw = xplicit_widget::XplicitWidget::new();
    let debug_text = debug_view.get_buffer().unwrap();
    let editor = ::xplicit::editor::Editor::new(FILENAME.to_string(), &xw, &debug_text);
    let h_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    h_pane.add2(&xw.drawing_area);
    h_pane.add1(&editor.text_view);

    let menu = menu::create_menu(move || {
                                     editor.save(&FILENAME.to_string());
                                 },
                                 || {
                                     gtk::main_quit();
                                 });

    let v_pane = gtk::Paned::new(gtk::Orientation::Vertical);
    v_pane.set_border_width(5);
    v_pane.add1(&h_pane);
    v_pane.add2(&debug_view);

    v_box.pack_start(&menu, false, false, 0);
    v_box.pack_start(&v_pane, true, true, 0);

    window.add(&v_box);
    window.show_all();

    v_pane.set_position(v_pane.get_allocated_height() * 80 / 100);
    h_pane.set_position(h_pane.get_allocated_width() * 50 / 100);

    gtk::main();
}
