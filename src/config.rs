use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::env;

pub struct Config {
    pub cfg: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Result<Config, ConfigError> {
        let mut path = if let Some(path) = env::home_dir() {
            path
        } else {
            return Err(ConfigError::NoHomeDir);
        };
        path.push(".wunderist");
        let f = try!(File::open(&path));
        let file = BufReader::new(&f);

        let mut config = HashMap::new();
        for (i, line) in file.lines().enumerate() {
            let l = try!(line);
            let v: Vec<&str> = l.split(':').map(str::trim).collect();
            if v.len() != 2 {
                return Err(ConfigError::Format(i + 1));
            }
            config.insert(v[0].to_string(), v[1].to_string());
        }
        Ok(Config { cfg: config })
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let mut path = if let Some(path) = env::home_dir() {
            path
        } else {
            return Err(ConfigError::NoHomeDir);
        };
        path.push(".wunderist");
        let mut f = try!(File::create(path));
        for (key, val) in self.cfg.iter() {
            try!(write!(&mut f, "{}: {}\n", key, val));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Format(usize),
    NoHomeDir,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Io(ref err) => err.fmt(f),
            ConfigError::Format(i) => {
                write!(f,
                       "format error at line {} in config file\n\
                       Enter wunderist config for more information",
                       i)
            }
            ConfigError::NoHomeDir => {
                write!(f, "No HOME directory!")
            }
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Io(ref err) => err.description(),
            ConfigError::Format(_) => "format error in config file",
            ConfigError::NoHomeDir => "no home directory",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::Io(ref err) => Some(err),
            ConfigError::Format(_) => None,
            ConfigError::NoHomeDir => None,
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}
