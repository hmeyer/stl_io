
extern crate gtk;
extern crate gdk;
extern crate xplicit;

use gtk::{Inhibit, WidgetSignals};
use gtk::traits::*;

use xplicit::xplicit_widget;
use xplicit::openscad;

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

    let v_pane = gtk::Paned::new(gtk::Orientation::Vertical);
    v_pane.set_border_width(5);
    let h_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    v_pane.add1(&h_pane);
    let debug_view = gtk::TextView::new();
    let debug_text = debug_view.get_buffer().unwrap();
    v_pane.add2(&debug_view);

    let code_view = gtk::TextView::new();
    let code_text = code_view.get_buffer().unwrap();
    h_pane.add1(&code_view);

    let xw = xplicit_widget::XplicitWidget::new();
    h_pane.add2(&xw.drawing_area);
    code_view.connect_key_release_event(move |code_view: &gtk::TextView,
                                              key: &gdk::EventKey|
                                              -> Inhibit {
        if key.get_keyval() == ::gdk::enums::key::F5 {
            // compile
            let code_buffer = code_view.get_buffer().unwrap();
            let code_text = code_buffer.get_text(&code_buffer.get_start_iter(),
                                                 &code_buffer.get_end_iter(),
                                                 true)
                                       .unwrap();
            let maybe_pgm = openscad::program(&code_text);
            if let Ok(pgm) = maybe_pgm {
                let mut out = format!("parsed : {:?}\n", pgm).into_bytes();
                let mut env = openscad::ast::Environment::new_with_primitives();
                let result = pgm.eval(&mut env, &mut out);
                out.append(&mut format!("\nexecuted : {:?}", result).into_bytes());

                if let openscad::ast::Value::Objects(objs) = result {
                    let union = xplicit::primitive::Union::from_vec(objs);
                    out.append(&mut format!("\n\nrendering : {:?}", union).into_bytes());
                    xw.renderer.borrow_mut().object = union;
                    xw.drawing_area.queue_draw();
                }
                debug_text.set_text(&String::from_utf8(out).unwrap());
            } else {
                debug_text.set_text(&format!("{:?}", maybe_pgm));
            }
        }
        Inhibit(false)
    });

    window.add(&v_pane);
    window.show_all();

    v_pane.set_position(v_pane.get_allocated_height() * 80 / 100);
    h_pane.set_position(h_pane.get_allocated_width() * 50 / 100);

    code_text.set_text(r#"
    translate([-.5,0,0]) sphere(.5);
    translate([ .5,0,0]) sphere(.8);
    "#);
    debug_view.set_wrap_mode(gtk::WrapMode::WordChar);

    gtk::main();
    code_view.set_wrap_mode(gtk::WrapMode::WordChar);

}
