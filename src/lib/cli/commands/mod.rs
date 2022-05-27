pub struct Commander;
use std::env::current_dir;
use std::fs;
use colored::Colorize;




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
[[routes]]
file_name = \"{}\"
to = \"{}\"\n
            ",
            file_name, to
        ).to_owned();

        write!(file, "{}", new_route)?;
        println!("Created {} route for {} file", to.bold().white(), file_name.bold().green());
        Ok(())
    }
}
