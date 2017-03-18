
extern crate gtk;
extern crate gdk;
extern crate xplicit;

use gtk::{FileChooserAction, FileChooserDialog, FileFilter, Inhibit, ResponseType};
use gtk::traits::*;
use xplicit::xplicit_widget;
use xplicit::menu;
use xplicit::settings;
use std::cell::RefCell;
use std::rc::Rc;

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
    let debug_scrolled_window = gtk::ScrolledWindow::new(None, None);
    let debug_view = gtk::TextView::new();
    debug_view.set_editable(false);
    debug_scrolled_window.add(&debug_view);
    debug_view.set_wrap_mode(gtk::WrapMode::WordChar);
    let xw = xplicit_widget::XplicitWidget::new();
    let debug_text = debug_view.get_buffer().unwrap();
    let editor = ::xplicit::editor::Editor::new(&xw, &debug_text);
    let h_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    h_pane.add2(&xw.drawing_area);
    h_pane.add1(&editor.widget);

    let editor_clone1 = editor.clone();
    let editor_clone2 = editor.clone();
    let editor_clone3 = editor.clone();
    let editor_clone4 = editor.clone();
    let window_clone1 = window.clone();
    let window_clone2 = window.clone();
    let window_clone3 = window.clone();
    let window_clone4 = window.clone();
    let filename = Rc::new(RefCell::new(String::new()));
    let filename_clone1 = filename.clone();
    let filename_clone2 = filename.clone();
    let menu = menu::create_menu(move || editor_clone1.tessellate(),
                                 move || {
                                     if let Some(path_str) = get_open_name(Some(&window_clone1)) {
                                         let mut f = filename.borrow_mut();
                                         *f = path_str;
                                         editor_clone2.open(&*f);
                                     }
                                 },
                                 move || {
                                     let mut f = filename_clone1.borrow_mut();
                                     if f.is_empty() {
                                         if let Some(path) = get_save_name(Some(&window_clone2)) {
                                             *f = path;
                                             editor_clone3.save(&*f);
                                         }
                                     } else {
                                         editor_clone3.save(&*f);
                                     }
                                 },
                                 move || {
                                     if let Some(path) = get_save_name(Some(&window_clone3)) {
                                         let mut f = filename_clone2.borrow_mut();
                                         *f = path;
                                         editor_clone4.save(&*f);
                                     }
                                 },
                                 move || settings::show_settings_dialog(Some(&window_clone4)),
                                 || gtk::main_quit());

    let v_pane = gtk::Paned::new(gtk::Orientation::Vertical);
    v_pane.set_border_width(5);
    v_pane.add1(&h_pane);
    v_pane.add2(&debug_scrolled_window);

    v_box.pack_start(&menu, false, false, 0);
    v_box.pack_start(&v_pane, true, true, 0);

    window.add(&v_box);
    window.show_all();

    v_pane.set_position(v_pane.get_allocated_height() * 80 / 100);
    h_pane.set_position(h_pane.get_allocated_width() * 50 / 100);

    gtk::main();
}

pub fn get_open_name<T: ::gtk::IsA<::gtk::Window>>(parent: Option<&T>) -> Option<String> {
    let dialog = FileChooserDialog::new(Some("Choose a file"), parent, FileChooserAction::Open);
    dialog.add_button("Open", ResponseType::Ok.into());
    dialog.add_button("Cancel", ResponseType::Cancel.into());
    dialog.set_select_multiple(false);
    let filter = FileFilter::new();
    filter.add_pattern("*.scad");
    dialog.add_filter(&filter);
    let res = dialog.run();
    let maybe_filename = dialog.get_filename();
    dialog.destroy();
    if res == ::gtk::ResponseType::Ok.into() {
        if let Some(path) = maybe_filename {
            if let Some(path_str) = path.to_str() {
                return Some(path_str.to_string());
            }
        }
    }
    None
}

pub fn get_save_name<T: ::gtk::IsA<::gtk::Window>>(parent: Option<&T>) -> Option<String> {
    let dialog = FileChooserDialog::new(Some("Choose a filename to Save"),
                                        parent,
                                        FileChooserAction::Save);
    dialog.add_button("Save", ResponseType::Ok.into());
    dialog.add_button("Cancel", ResponseType::Cancel.into());
    let filter = FileFilter::new();
    filter.add_pattern("*.scad");
    dialog.add_filter(&filter);
    let res = dialog.run();
    let maybe_filename = dialog.get_filename();
    dialog.destroy();
    if res == ::gtk::ResponseType::Ok.into() {
        if let Some(path) = maybe_filename {
            if let Some(path_str) = path.to_str() {
                return Some(path_str.to_string());
            }
        }
    }
    None
}
