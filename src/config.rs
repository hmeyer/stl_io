use rustc_serialize::Encodable;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use toml::{Decoder, Encoder, Parser, Value};
use rustc_serialize::Decodable;

const CONFIG_FILENAME: &'static str = ".xplicit";


#[derive(RustcDecodable, RustcEncodable)]
pub struct ConfigData {
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
enum ConfigError {
    Io(::std::io::Error),
    Dec(::toml::DecodeError),
    Enc(::toml::Error)
}

impl ConfigData {
    fn path() -> Result<::std::path::PathBuf, ConfigError> {
        let mut path = match ::std::env::home_dir() {
            Some(p) => p,
            None => try!(::std::env::current_dir().map_err(ConfigError::Io))
        };
        path.push(CONFIG_FILENAME);
        Ok(path)
    }
    fn get_toml() -> Result<Self, ConfigError> {
        let path = try!(ConfigData::path());
        let f = try!(File::open(path).map_err(ConfigError::Io));
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();
        let _ = try!(reader.read_to_string(&mut buffer).map_err(ConfigError::Io));
        let mut parser = Parser::new(&buffer);
        match parser.parse() {
            Some(value) => {
                let mut decoder = Decoder::new(Value::Table(value));
                let config = try!(Decodable::decode(&mut decoder).map_err(ConfigError::Dec));
                Ok(config)
            },
            None => {
                Err(ConfigError::Io(::std::io::Error::new(::std::io::ErrorKind::InvalidInput,
                                          join(parser.errors
                                                     .iter()
                                                     .map(|e| format!("{}", e))
                                                     .collect(),
                                               ", "))))
            }
        }
    }
    pub fn new() -> ConfigData {
        match ConfigData::get_toml() {
            Ok(c) => c,
            Err(e) => {
                println!("error reading config: {:?}", e);
                ConfigData { tessellation_resolution: 0.12 }
            }
        }
    }

    fn put_toml(&mut self) -> Result<(), ConfigError> {
        let mut e = Encoder::new();
        try!(self.encode(&mut e).map_err(ConfigError::Enc));
        let path = try!(ConfigData::path());
        let file = try!(File::create(path).map_err(ConfigError::Io));
        let mut writer = BufWriter::new(file);
        try!(writer.write(format!("{}", Value::Table(e.toml)).as_bytes()).map_err(ConfigError::Io));
        Ok(())
    }
}

impl ::std::ops::Drop for ConfigData {
    fn drop(&mut self) {
        match self.put_toml() {
            Ok(_) => {},
            Err(e) => println!("error writing config: {:?}", e),
        }
    }
}
