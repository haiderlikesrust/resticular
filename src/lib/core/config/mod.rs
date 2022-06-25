use super::{fs::reader::Reader, IntoInner};
use crate::error::{self, ConfigError, Error};
use serde_derive::Deserialize;
use std::{env::current_dir, fs};
use toml::from_str;
use std::io::Write;
pub const CONFIG_PATH: &str = "resticular.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub out_dir: String,
    pub source: String,
    pub lazy_images: Option<bool>,
    pub routes: Vec<Route>,
    pub global_css: Option<bool>
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Route {
    pub to: String,
    pub file_name: String,
}

impl Config {
    pub fn read_config() -> Result<Self, Error> {
        let current_dir = current_dir()?;
        let path = format!("{}/resticular.toml", current_dir.to_str().unwrap());
        let config_file = Reader::new(path.into());
        let config_content = config_file.reader()?.into_inner().into_inner();
        let config = Config::parse(&config_content)?;
        Ok(config)
    }
    fn parse(config_content: &str) -> Result<Config, ConfigError> {
        let a: Config = from_str(config_content)?;
        Ok(a)
    }

    pub fn fix(mut self) -> Result<Self, Error> {
        for route in &mut self.routes {
            if route.file_name.starts_with('/')
                || route.file_name.ends_with('/')
            {
                return Err(Error::ConfigFileError(
                    error::ConfigError::ConfigFileParseError(r"The field `file_name` in your `resticular.toml` either starts with `/` or ends `/` or does not end with `.html`. "
                    .to_string()))
                );
            }
            route.file_name = format!("{}/{}", self.out_dir, route.file_name);
        }
        Ok(self)
    }

    pub fn new_route(file_name: String, to: String) -> Result<(), Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/{}", current_dir()?.to_str().unwrap(), "resticular.toml"))?;
        let new_route = format!(
"\n
[[routes]]
file_name = \"{}\"
to = \"{}\"\n
            ",
            file_name, to
        );

        write!(file, "{}", new_route)?;
        Ok(())
    }
}


