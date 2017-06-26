use editor;
use gtk::{FileChooserAction, FileChooserDialog, FileFilter, Inhibit, ResponseType};
use gtk::traits::*;
use menu;
use object_widget;
use settings;
use std::cell::RefCell;
use std::rc::Rc;
use stl_io::write_stl;

macro_rules! clone {
    ($($n:ident),+; || $body:stmt) => (
        {
            $( let $n = $n.clone(); )+
            move || { $body }
        }
    );
    ($($n:ident),+; |$($p:ident),+| $body:stmt) => (
        {
            $( let $n = $n.clone(); )+
            move |$($p),+| { $body }
        }
    );
}

pub fn create_window() -> ::gtk::Window {
    let window = ::gtk::Window::new(::gtk::WindowType::Toplevel);

    window.set_default_size(640, 480);

    window.connect_delete_event(|_, _| {
                                    ::gtk::main_quit();
                                    Inhibit(false)
                                });

    let v_box = ::gtk::Box::new(::gtk::Orientation::Vertical, 0);
    let debug_scrolled_window = ::gtk::ScrolledWindow::new(None, None);
    let debug_view = ::gtk::TextView::new();
    debug_view.set_editable(false);
    debug_scrolled_window.add(&debug_view);
    debug_view.set_wrap_mode(::gtk::WrapMode::WordChar);
    let xw = object_widget::ObjectWidget::new();
    let debug_text = debug_view.get_buffer().unwrap();
    let editor = editor::Editor::new(&xw, &debug_text);
    let h_pane = ::gtk::Paned::new(::gtk::Orientation::Horizontal);
    h_pane.add2(&xw.drawing_area);
    h_pane.add1(&editor.widget);

    let filename = Rc::new(RefCell::new(String::new()));

    let menu = menu::create_menu(clone!(editor; || {
                                     editor.tessellate();
                                 }),
                                 clone!(window, editor, filename; || {
                                     if let Some(path_str) = get_open_name(Some(&window)) {
                                         let mut f = filename.borrow_mut();
                                         *f = path_str;
                                         editor.open(&*f);
                                     }
                                 }),
                                 clone!(window, editor, filename; || {
                                     let mut f = filename.borrow_mut();
                                     if f.is_empty() {
                                         if let Some(path) = get_save_name(Some(&window),
                                                                           "*.lua") {
                                             *f = path;
                                             editor.save(&*f);
                                         }
                                     } else {
                                         editor.save(&*f);
                                     }
                                 }),
                                 clone!(window, editor, filename; || {
                                     if let Some(path) = get_save_name(Some(&window),
                                                                       "*.lua") {
                                         let mut f = filename.borrow_mut();
                                         *f = path;
                                         editor.save(&*f);
                                     }
                                 }),
                                 clone!(window; || settings::show_settings_dialog(Some(&window))),
                                 clone!(window, editor; || {
                                     let maybe_mesh = editor.tessellate();
                                     if let Some(mesh) = maybe_mesh {
                                         if let Some(path) = get_save_name(Some(&window),
                                                                           "*.stl") {
                                             let stl_mesh = mesh.faces.iter().enumerate().map(|(i, f)| {
                                                 let normal = mesh.normal32(i);
                                                 ::stl_io::Triangle{ normal:[normal[0], normal[1], normal[2]],
                                                     vertices: [mesh.vertex32(f[0]),
                                                      mesh.vertex32(f[1]),
                                                      mesh.vertex32(f[2])]}
                                             }).collect::<Vec<_>>();
                                             println!("writing STL ({:?}): {:?}",
                                                      path,
                                                      write_stl(&path, stl_mesh.iter()));
                                         }
                                     }
                                 }),
                                 || ::gtk::main_quit());

    let v_pane = ::gtk::Paned::new(::gtk::Orientation::Vertical);
    v_pane.set_border_width(5);
    v_pane.add1(&h_pane);
    v_pane.add2(&debug_scrolled_window);

    v_box.pack_start(&menu, false, false, 0);
    v_box.pack_start(&v_pane, true, true, 0);

    window.add(&v_box);
    window.show_all();

    v_pane.set_position(v_pane.get_allocated_height() * 80 / 100);
    h_pane.set_position(h_pane.get_allocated_width() * 50 / 100);

    return window;

}

fn get_open_name<T: ::gtk::IsA<::gtk::Window>>(parent: Option<&T>) -> Option<String> {
    let dialog = FileChooserDialog::new(Some("Choose a file"), parent, FileChooserAction::Open);
    dialog.add_button("Open", ResponseType::Ok.into());
    dialog.add_button("Cancel", ResponseType::Cancel.into());
    dialog.set_select_multiple(false);
    let filter = FileFilter::new();
    filter.add_pattern("*.lua");
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

fn get_save_name<T: ::gtk::IsA<::gtk::Window>>(parent: Option<&T>,
                                               pattern: &str)
                                               -> Option<String> {
    let dialog = FileChooserDialog::new(Some("Choose a filename to Save"),
                                        parent,
                                        FileChooserAction::Save);
    dialog.add_button("Save", ResponseType::Ok.into());
    dialog.add_button("Cancel", ResponseType::Cancel.into());
    let filter = FileFilter::new();
    filter.add_pattern(pattern);
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
