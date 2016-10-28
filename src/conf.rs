
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use serde_json;

const CONF_FILE_NAME: &'static str = "/.clack.conf.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
    pub tokens: Vec<String>,
}

pub fn load() -> Result<Conf, String> {
    let home = env::home_dir().unwrap().to_str().unwrap().to_string();
    match File::open(home+CONF_FILE_NAME) {
        Ok(mut f) => {
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => {
                    match serde_json::from_str(&*s) {
                        Ok(c) => Ok(c),
                        Err(e) => Err(format!("unable to deserialize conf: {}", e.description()))
                    }
                },
                Err(e) => return Err(format!("unable to read file: {}", e.description()))
            }
        },
        Err(e) => return Err(format!("enable to open file: {}", e.description()))
    }
}
