use rustc_serialize::Encodable;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use toml::{Decoder, Encoder, Parser, Value};
use rustc_serialize::Decodable;
use gtk::{BoxExt, ContainerExt, DialogExt, WidgetExt};

const SETTINGS_FILENAME: &'static str = ".xplicit";

pub fn show_settings_dialog<T: ::gtk::IsA<::gtk::Window>>(parent: Option<&T>) {
    let mut data = SettingsData::new();

    let dialog = ::gtk::Dialog::new_with_buttons(Some("Settings"), parent, ::gtk::DIALOG_MODAL,
    &[("OK", ::gtk::ResponseType::Ok.into()), ("Cancel", ::gtk::ResponseType::Cancel.into())]);
    // TODO: use rustc_serialize::Encodable to generate settings items
    let h_box = ::gtk::Box::new(::gtk::Orientation::Horizontal, 0);
    let tessellation_resolution_label = ::gtk::Label::new_with_mnemonic(Some("_tessellation resolution"));
    let tessellation_resolution = ::gtk::SpinButton::new_with_range(0.0001, 1000., 0.01);
    tessellation_resolution.set_digits(3);
    tessellation_resolution.set_value(data.tessellation_resolution);

    h_box.pack_start(&tessellation_resolution_label, true, false, 5);
    h_box.pack_start(&tessellation_resolution, true, false, 5);

    dialog.get_content_area().add(&h_box);
    dialog.show_all();
    let ret = dialog.run();

    if ret == ::gtk::ResponseType::Ok.into()     {
        data.tessellation_resolution = tessellation_resolution.get_value();
        data.save();
    }

    dialog.destroy();
    println!("done settings: {}", ret);
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct SettingsData {
    tessellation_resolution: f64,
}

fn join<S: ToString>(l: Vec<S>, sep: &str) -> String {
    l.iter().fold("".to_string(), |a, b| {
        if a.len() > 0 {
            a + sep
        } else {
            a
        }
    } + &b.to_string())
}

#[derive(Debug)]
enum SettingsError {
    Io(::std::io::Error),
    Dec(::toml::DecodeError),
    Enc(::toml::Error)
}

impl SettingsData {
    fn path() -> Result<::std::path::PathBuf, SettingsError> {
        let mut path = match ::std::env::home_dir() {
            Some(p) => p,
            None => try!(::std::env::current_dir().map_err(SettingsError::Io))
        };
        path.push(SETTINGS_FILENAME);
        Ok(path)
    }
    fn get_toml() -> Result<Self, SettingsError> {
        let path = try!(SettingsData::path());
        let f = try!(File::open(path).map_err(SettingsError::Io));
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();
        let _ = try!(reader.read_to_string(&mut buffer).map_err(SettingsError::Io));
        let mut parser = Parser::new(&buffer);
        match parser.parse() {
            Some(value) => {
                let mut decoder = Decoder::new(Value::Table(value));
                let settings = try!(Decodable::decode(&mut decoder).map_err(SettingsError::Dec));
                Ok(settings)
            },
            None => {
                Err(SettingsError::Io(::std::io::Error::new(::std::io::ErrorKind::InvalidInput,
                                          join(parser.errors
                                                     .iter()
                                                     .map(|e| format!("{}", e))
                                                     .collect(),
                                               ", "))))
            }
        }
    }
    pub fn new() -> SettingsData {
        match SettingsData::get_toml() {
            Ok(c) => c,
            Err(e) => {
                println!("error reading settings: {:?}", e);
                SettingsData { tessellation_resolution: 0.12 }
            }
        }
    }

    fn put_toml(&self) -> Result<(), SettingsError> {
        let mut e = Encoder::new();
        try!(self.encode(&mut e).map_err(SettingsError::Enc));
        let path = try!(SettingsData::path());
        let file = try!(File::create(path).map_err(SettingsError::Io));
        let mut writer = BufWriter::new(file);
        try!(writer.write(format!("{}", Value::Table(e.toml)).as_bytes()).map_err(SettingsError::Io));
        Ok(())
    }

    pub fn save(&self) {
        match self.put_toml() {
            Ok(_) => {},
            Err(e) => println!("error writing settings: {:?}", e),
        }
    }
}
