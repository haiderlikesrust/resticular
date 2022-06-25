use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use serde_json::{json, Value, Map};
use tera::Context;
use crate::error::Error;

use super::config::Config;

pub struct PageData;

impl PageData {
    pub fn extract(data: &mut Context) -> Result<(), Error> {
        let config = Config::read_config()?;
        match Path::new(&format!("{}/data.json", config.source)).exists() {
            true => {
                let mut buffer = String::new();
                let f = File::open(&format!("{}/data.json", config.source)).unwrap();
                let mut reader = BufReader::new(f);
                reader.read_to_string(&mut buffer).unwrap();
                let parsed: HashMap<String, String> = serde_json::from_str(&buffer).unwrap();
                parsed.iter()
                    .for_each(|v| {
                        data.insert(v.0, v.1)
                    });
                Ok(())
            }
            _ => Ok(()),
        }
    }
}