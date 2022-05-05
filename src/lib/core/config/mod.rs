use super::{fs::reader::Reader, IntoInner};
use crate::error::Error;
use serde_derive::Deserialize;
use std::env::current_dir;
use toml::from_str;
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub out_dir: String,
    pub dir: String,
    pub lazy_images: Option<bool>
}

impl Config {
    pub fn read_config() -> Result<Self, Error> {
        let current_dir = current_dir()?;
        let path = format!("{}/resticular.toml", current_dir.to_str().unwrap());
        let config_file = Reader::new(path.into());
        let config_content = config_file.reader()?.into_inner().into_inner();
        let config: Config = from_str(&config_content).unwrap();
        Ok(config)
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
}
