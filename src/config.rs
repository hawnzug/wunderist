use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;

pub struct Config {
    pub cfg: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let path = Path::new(".wunderist");
        let f = File::open(&path).unwrap();
        let file = BufReader::new(&f);

        let mut config = HashMap::new();
        for (i, line) in file.lines().enumerate() {
            let l = line.unwrap();
            let v: Vec<&str> = l.split(':').map(str::trim).collect();
            if v.len() != 2 {
                return Err(format!("config error: more than one colons at line {}", i + 1));
            }
            config.insert(v[0].to_string(), v[1].to_string());
        }
        Ok(Config { cfg: config })
    }

    pub fn save(&self) {
        let mut f = File::create(".wunderist").unwrap();
        for (key, val) in self.cfg.iter() {
            write!(&mut f, "{}: {}", key, val);
        }
    }
}
