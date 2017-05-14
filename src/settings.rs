use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use gtk::{BoxExt, ContainerExt, DialogExt, SpinButton, SpinButtonSignals, WidgetExt};

const SETTINGS_FILENAME: &'static str = ".truescad";

macro_rules! add_setting {
    ($field :ident, $data :expr) => {{
        let data_clone = $data.clone();
        let h_box = ::gtk::Box::new(::gtk::Orientation::Horizontal, 0);
        let label = ::gtk::Label::new_with_mnemonic(Some(stringify!($field)));
        let setting = SpinButton::new_with_range(0.0001, 1000., 0.01);
        setting.set_digits(3);
        setting.set_value($data.borrow().$field);
        setting.connect_value_changed(move |f: &SpinButton| {
            data_clone.borrow_mut().$field = f.get_value();
        });
        h_box.pack_start(&label, true, false, 5);
        h_box.pack_start(&setting, true, false, 5);
        h_box
    }};
}


pub fn show_settings_dialog<T: ::gtk::IsA<::gtk::Window>>(parent: Option<&T>) {
    let data = Rc::new(RefCell::new(SettingsData::new()));

    let dialog = ::gtk::Dialog::new_with_buttons(Some("Settings"),
                                                 parent,
                                                 ::gtk::DIALOG_MODAL,
                                                 &[("OK", ::gtk::ResponseType::Ok.into()),
                                                   ("Cancel", ::gtk::ResponseType::Cancel.into())]);
    // TODO: use rustc_serialize::Encodable to generate settings items
    dialog.get_content_area().add(&add_setting!(tessellation_resolution, &data));
    dialog.get_content_area().add(&add_setting!(tessellation_error, &data));
    dialog.get_content_area().add(&add_setting!(fade_range, &data));
    dialog.get_content_area().add(&add_setting!(r_multiplier, &data));

    dialog.show_all();
    let ret = dialog.run();

    if ret == ::gtk::ResponseType::Ok.into() {
        data.borrow().save();
    }
    dialog.destroy();
}

#[derive(Serialize, Deserialize)]
pub struct SettingsData {
    pub tessellation_resolution: f64,
    pub tessellation_error: f64,
    pub fade_range: f64,
    pub r_multiplier: f64,
}

#[derive(Debug)]
enum SettingsError {
    Io(::std::io::Error),
    Dec(::toml::de::Error),
    Enc(::toml::ser::Error),
}

impl SettingsData {
    fn path() -> Result<::std::path::PathBuf, SettingsError> {
        let mut path = match ::std::env::home_dir() {
            Some(p) => p,
            None => try!(::std::env::current_dir().map_err(SettingsError::Io)),
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
        return ::toml::from_str(&buffer).map_err(SettingsError::Dec);
    }
    pub fn new() -> SettingsData {
        match SettingsData::get_toml() {
            Ok(c) => c,
            Err(e) => {
                println!("error reading settings: {:?}", e);
                SettingsData {
                    tessellation_resolution: 0.12,
                    tessellation_error: 2.,
                    fade_range: 0.1,
                    r_multiplier: 1.0,
                }
            }
        }
    }

    fn put_toml(&self) -> Result<(), SettingsError> {
        let toml_str = try!(::toml::to_string(self).map_err(SettingsError::Enc));
        let path = try!(SettingsData::path());
        let file = try!(File::create(path).map_err(SettingsError::Io));
        let mut writer = BufWriter::new(file);
        try!(writer.write(toml_str.as_bytes()).map_err(SettingsError::Io));
        Ok(())
    }

    pub fn save(&self) {
        match self.put_toml() {
            Ok(_) => {}
            Err(e) => println!("error writing settings: {:?}", e),
        }
    }
}
