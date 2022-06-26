use crate::error::{Error, TemplateError};
use serde::de::Error as DeError;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tera::Context;
use colored::Colorize;
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
                let parsed = serde_json::from_str::<HashMap<String, String>>(&buffer);
                match parsed {
                    Ok(p) => {
                        p.iter().for_each(|v| data.insert(v.0, v.1));
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(Error::TemplateError(
                            TemplateError::InvalidValueTypeInDataFile(format!(
                                "An invalid data type was used in the `{}` file, error returned [{}]",
                                "data.json".red().bold(),
                                e.to_string()
                            )),
                        ))
                    }
                }
            }
            _ => Ok(()),
        }
    }
}
