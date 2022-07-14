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
                    if let Some(pre_commands) = &command.pre_commands {
                        for pre_command in pre_commands {
                            let pre_command: &'static str =
                                Box::leak(pre_command.clone().into_boxed_str());
                            alert_cli!(
                                format!("Executing Precommand: {}", &pre_command).green(),
                                bold
                            );
                            let output = duct_sh::sh(&pre_command).read();
                            match output {
                                Ok(s) => {
                                    println!("{s}")
                                }
                                Err(_) => {
                                    alert_cli!(
                                        format!(
                                            "Pre Command: {} has an error, please look into it.",
                                            pre_command
                                        )
                                        .red(),
                                        bold
                                    );
                                }
                            }
                        }
                    }
                    let cmd_clone: &'static str =
                        Box::leak(command.clone().command.into_boxed_str());

                    let output = duct_sh::sh(&cmd_clone).read();
                    match output {
                        Ok(s) => {
                            println!("{s}")
                        }
                        Err(_) => println!("Command Errored out"),
                    }
                }
            }
        }

        Ok(())
    }

    pub fn commands() -> Result<(), Error> {
        let config = Config::read_config()?;
        match config.command {
            Some(c) => {
                for command in c {
                    match command.pre_commands {
                        Some(pre) => {
                            let mut pre_commands = String::new();
                            for p in &pre {
                                if pre.len() == 1 {
                                    pre_commands.push_str(
                                        &format!("{p} ").bold().green(),
                                    );
                                } else if pre.last().unwrap() == p {
                                    pre_commands.push_str(
                                        &format!("{p} ").bold().green(),
                                    );
                                } else {
                                    pre_commands.push_str(
                                    &format!("{p} {}", format!(">> ").red()).bold().green(),
                                );
                                }
                                
                            }
                            alert_cli!(
                                format!(
                                    "Name: {}\nCommand: {}\nPreCommands:  {pre_commands}\n\n",
                                    command.name, command.command
                                ).green(),
                                bold
                            );
                        }
                        None => {
                            alert_cli!(
                                format!(
                                    "Name: {}\nCommand: {}\nPreCommands:  None\n\n",
                                    command.name, command.command
                                )
                                .green(),
                                bold
                            );
                        }
                    }
                }
            }
            None => {
                alert_cli!(
                    format!("No secondary commands. Add them by editing the config file.").green(),
                    bold
                )
            }
        }

        Ok(())
    }
}
