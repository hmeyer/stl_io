use openscad;
use primitive;
use xplicit_widget;
use gtk::{Inhibit, WidgetSignals};
use gtk::traits::*;


pub struct Editor {
    pub text_view: ::gtk::TextView,
}


impl Editor {
    pub fn new(xw: &xplicit_widget::XplicitWidget, debug_buffer: &::gtk::TextBuffer) -> Editor {
        let tv = ::gtk::TextView::new();
        // TODO: Find out why this causes a non-draw on startup.
        // tv.set_wrap_mode(::gtk::WrapMode::WordChar);
        tv.get_buffer().unwrap().set_text(r#"
            difference() {
                union() {
                    translate([-.5, 0, 0]) sphere(.5);
                    translate([ .5, 0, 0]) sphere(.8);
                }
                translate([0, 0, .5]) sphere(.5);
            }
            "#);
        let renderer = xw.renderer.clone();
        let drawing_area = xw.drawing_area.clone();
        let debug_buffer_clone = debug_buffer.clone();
        tv.connect_key_release_event(move |tv: &::gtk::TextView,
                                           key: &::gdk::EventKey|
                                           -> Inhibit {
            if key.get_keyval() == ::gdk::enums::key::F5 {
                // compile
                let code_buffer = tv.get_buffer().unwrap();
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
                        let union = primitive::Union::from_vec(objs, 0.);
                        out.append(&mut format!("\n\nrendering : {:?}", union).into_bytes());
                        renderer.borrow_mut().object = union;
                        drawing_area.queue_draw();
                    }
                    debug_buffer_clone.set_text(&String::from_utf8(out).unwrap());
                } else {
                    debug_buffer_clone.set_text(&format!("{:?}", maybe_pgm));
                }
            }
            Inhibit(false)
        });
        Editor { text_view: tv }
    }
}
