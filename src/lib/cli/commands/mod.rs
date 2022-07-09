pub struct Commander;
use cmd_lib::{run_cmd, run_fun};
use colored::Colorize;
use std::env::current_dir;
use std::fs::{self, create_dir, File};
use std::process::Command;

use crate::core::config::Config;
use crate::error::Error;
use std::io::Write;
const DEF_CONFIG: &str = r#"
# `source` is the folder where you will work, create pages, etc.
source = "source"
# `out_dir` this is the folder where  resticular will spit the files.
out_dir = "dist"
# Routes will be manually added by the user.
"#;

impl Commander {
    pub fn new_route(file_name: String, to: String) -> Result<(), Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!(
                "{}/{}",
                current_dir()?.to_str().unwrap(),
                "resticular.toml"
            ))?;
        let new_route = format!(
            "\n
[[routes]]
file_name = \"{}\"
to = \"{}\"\n
            ",
            file_name, to
        );

        write!(file, "{}", new_route)?;
        println!(
            "Created {} route for {} file",
            to.bold().white(),
            file_name.bold().green()
        );
        Ok(())
    }

    pub fn new_cmd(name: &str) -> Result<(), Error> {
        let current_path = current_dir()?;
        let project_path = format!("{}/{name}", &current_path.to_str().unwrap());
        create_dir(&project_path)?;
        create_dir(format!("{project_path}/source"))?;
        create_dir(format!("{project_path}/source/assets"))?;
        create_dir(format!("{project_path}/source/components"))?;
        create_dir(format!("{project_path}/dist"))?;
        let mut config = File::create(format!("{project_path}/resticular.toml"))?;
        config.write_all(DEF_CONFIG.as_bytes())?;
        println!("Created new resticular project `{}`.", name.bold().green());

        Ok(())
    }
    pub fn execute_command(name: &str) -> Result<(), Error> {
        let config = Config::read_config()?;
        if let Some(commands) = config.command {
            for command in commands {
                if name == command.name {
                    let cmd_clone: &'static str =
                        Box::leak(command.clone().command.into_boxed_str());

                    let output = duct_sh::sh(&cmd_clone).read();
                    match output {
                        Ok(s) => {
                            println!("{s}")
                        }, 
                        Err(_) => println!("Command Errored out")
                    } 
                
                }
            }
        }

        Ok(())
    }
}
