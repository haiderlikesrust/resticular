use crate::{
    core::{config::Config, fs::build_size},
    error::Error,
    process, sub_process,
};
use clap::{arg, ArgMatches, Command, AppSettings};
use colored::*;

use self::commands::Commander;
#[macro_export]
macro_rules! alert_cli {
    ($msg: expr, $color_or_font: ident) => {
        println!("{}", $msg.$color_or_font())
    };
}
pub struct Cli;
pub mod commands;

impl Cli {
    pub fn start(self) -> Result<(), Error> {
        self.cmd()
    }

    fn cmd(self) -> Result<(), Error> {
        let app = Command::new("resticular")
            .about("Easy and flexible SSG.")
            .author("Haider Ali")
            .arg_required_else_help(true)
            .subcommand(Command::new("build").about("Builds the source directory."))
            .subcommand(
                Command::new("serve").about("Builds the source folder and runs the dev server."),
            )
            .subcommand(
                Command::new("add").about("Adds something.").subcommand(
                    Command::new("route")
                        .about("Adds a route")
                        .arg(
                            arg!(--to [TO] "The path of the route on the browser.")
                                .default_value(""),
                        )
                        .arg(arg!(--name <NAME> "Name of the file wirh extension")),
                ),
            )
            .subcommand(Command::new("start").about("Starts building and starts the server"));
        let matches = app.get_matches();
        self.figure_out_matches(&matches)?;
        Ok(())
    }

    fn figure_out_matches(self, matches: &ArgMatches) -> Result<(), Error> {
        let subcommands = matches.subcommand();
        match subcommands {
            Some(("build", _)) => {
                let config = Config::read_config()?;
                sub_process(&config.dir)?
            }
            Some(("add", arg)) => {
                let sub_commands = arg.subcommand();
                match sub_commands {
                    Some(("route", matches)) => {
                        let to = matches.value_of("to").unwrap();
                        let file_name = matches.value_of("name").unwrap();
                        Commander::new_route(file_name.to_owned(), to.to_owned())?;
                    }
                    _ => (),
                }
            }
            Some(("start", _)) => {
                process()?;
            }

            _ => (),
        }

        Ok(())
    }
}
