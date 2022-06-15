use crate::{
    core::{config::Config, fs::build_size},
    error::Error,
    process, sub_process,
};
use clap::{AppSettings, Parser, Subcommand};
use colored::*;

use self::commands::Commander;
#[macro_export]
macro_rules! alert_cli {
    ($msg: expr, $color_or_font: ident) => {
        println!("{}", $msg.$color_or_font())
    };
}
pub mod commands;

/// Easy and flexible SSG
#[derive(Parser)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
#[clap(author = "Haider Ali")]
enum Cli {
    /// Builds the source dir
    Build,

    /// Builds the source dir and runs the dev server
    Serve,

    /// Builds the source dir and starts the server
    Start,

    /// Adds something
    #[clap(subcommand)]
    Add(AddCommand),
    // Creates new resticular project.
    New {
        name: String
    }
    
}


#[derive(Subcommand)]
enum AddCommand {
    /// Adds a route
    Route {
        /// Path of the route on the browser
        #[clap(long, default_value = "")]
        to: String,

        /// Name of the file with extension
        #[clap(long)]
        name: String,
    },
}

pub fn start() -> Result<(), Error> {
    let args = Cli::parse();

    match args {
        Cli::Build => {
            let config = Config::read_config()?;
            sub_process(&config.source)?;
        }
        Cli::Add(AddCommand::Route { to, name }) => {
            Commander::new_route(name, to)?;
        }
        Cli::Start => {
            process()?;
        },
        Cli::New {name} => {
           Commander::new_cmd(&name)?;
        }
        _ => (),
    }

    Ok(())
}
