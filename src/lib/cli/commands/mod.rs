pub struct Commander;
use std::env::current_dir;
use std::fs;

use crate::core::config::CONFIG_PATH;
use crate::core::fs::reader::{FileContent, Reader, Writer};
use crate::core::fs::Data;
use crate::error::Error;
use std::io::Write;

impl Commander {
    pub fn new_route(file_name: String, to: String) -> Result<(), Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/{}", current_dir()?.to_str().unwrap(), "resticular.toml"))?;
        let new_route = format!(
            "\n
[[routes]]\n
file_name = \"{}\"\n
to = \"{}\"\n
            ",
            file_name, to
        ).trim().to_owned();

        write!(file, "{}", new_route)?;
        Ok(())
    }
}
