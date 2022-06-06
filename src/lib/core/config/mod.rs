use super::{fs::reader::Reader, IntoInner};
use crate::error::{self, ConfigError, Error};
use serde_derive::Deserialize;
use std::env::current_dir;
use toml::from_str;
pub const CONFIG_PATH: &str = "resticular.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub out_dir: String,
    pub dir: String,
    pub lazy_images: Option<bool>,
    pub routes: Vec<Route>,
}
#[derive(Debug, Deserialize, Clone)]
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
        let a: Config = from_str(&config_content)?;
        Ok(a)
    }

    pub fn fix(mut self) -> Result<Self, Error> {
        for route in &mut self.routes {
            if route.file_name.starts_with('/')
                || route.file_name.ends_with('/')
                || !route.file_name.ends_with(".html")
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
}

#[cfg(test)]
mod test {
    use super::Config;
    #[test]
    fn check_config() {
        let c = Config::read_config().unwrap();
        assert_eq!(c.dir, "dist")
    }

    #[test]
    fn check_file() {
        let _c = Config::read_config().unwrap();
    }
}
