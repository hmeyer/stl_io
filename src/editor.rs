use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use openscad;
use primitive;
use xplicit_widget;
use gtk::Inhibit;
use gtk::traits::*;


pub struct Editor {
    pub text_view: ::gtk::TextView,
}


impl Editor {
    pub fn new(input_filename: String,
               xw: &xplicit_widget::XplicitWidget,
               debug_buffer: &::gtk::TextBuffer)
               -> Editor {
        let tv = ::gtk::TextView::new();
        // TODO: Find out why this causes a non-draw on startup.
        // tv.set_wrap_mode(::gtk::WrapMode::WordChar);
        let open_result = File::open(&input_filename);
        if let Ok(f) = open_result {
            let reader = BufReader::new(f);
            let mut buffer = String::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    buffer.push_str(&line);
                    buffer.push_str("\n");
                }
            }
            tv.get_buffer().unwrap().set_text(&buffer);
        } else {
            println!("could not open {:?}: {:?}", &input_filename, open_result);
        }

        let renderer = xw.renderer.clone();
        let drawing_area = xw.drawing_area.clone();
        let debug_buffer_clone = debug_buffer.clone();
        tv.connect_key_release_event(move |tv: &::gtk::TextView,
                                           key: &::gdk::EventKey|
                                           -> Inhibit {
            match key.get_keyval() {
                ::gdk::enums::key::F5 => {
                    // compile
                    let code_buffer = tv.get_buffer().unwrap();
                    let code_text = code_buffer.get_text(&code_buffer.get_start_iter(),
                                                         &code_buffer.get_end_iter(),
                                                         true)
                                               .unwrap();
                    let maybe_pgm = openscad::program(&code_text);
                    if let Ok(pgm) = maybe_pgm {
                        let mut out = format!("parsed : {:?}\n", pgm).into_bytes();
                        let mut env = openscad::ast::Environment::new();
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
                ::gdk::enums::key::F2 => {
                    let open_result = File::create(&input_filename);
                    if let Ok(f) = open_result {
                        let code_buffer = tv.get_buffer().unwrap();
                        let code_text = code_buffer.get_text(&code_buffer.get_start_iter(),
                                                             &code_buffer.get_end_iter(),
                                                             true)
                                                   .unwrap();
                        let mut writer = BufWriter::new(f);
                        let write_result = writer.write(code_text.as_bytes());
                        println!("writing {:?}: {:?}", &input_filename, write_result);
                    } else {
                        println!("opening for write {:?}: {:?}", &input_filename, open_result);
                    }
                }
                _ => {
                    // println!("unbound key release: {:?}", x);
                }
            }
            Inhibit(false)
        });
        Editor { text_view: tv }
    }
}
