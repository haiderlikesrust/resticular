use crate::core::components::Component;
use crate::core::fs::watcher::watch;

use crate::core::http::MsgHandler;
use crate::core::JoinHandler;

use std::thread;

use crate::core::config::Config;
use crate::core::fs::reader::{read, FolderBuilder};

use crate::core::html::HtmlWriter;

use error::Error;
use tokio::runtime::Runtime;
pub mod cli;
/// Module on which resticular's functionality depends.
pub mod core;
pub mod error;
pub mod prelude;
use colored::{Colorize};

#[cfg(test)]
pub mod tests;
use crate::core::fs::reader::start_convert_and_parse;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[macro_export]
macro_rules! handle_thread_error_with_error {
    ($func: expr, $error: expr) => {
        match $func {
            Ok(_) => "Passed".to_owned(),
            Err(_) => {
                panic!("Error occured in a thread: {}", $error)
            }
        }
    };
}

#[macro_export]
macro_rules! handle_thread_error {
    ($func: expr) => {
        match $func {
            Ok(_) => "Passed".to_owned(),
            Err(a) => {
                panic!("Error occured in a thread: {}", a)
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum EyeKeeper {
    Changed,
    Unchanged,
}

pub enum ProcessIndicator {
    True,
    False,
}

fn sub_process(dir: &str) -> Result<(), Error> {
    let components = Component::read()?;
    let global_css = Config::read_config()?.global_css;
    let _config = Config::read_config()?;
    alert_cli!(format!("\u{1F4C1} | Creating the file {}.", "reader".green()), bold);
    let f = read(dir)?;
    alert_cli!(
        format!("\u{1F4C1} | Reading files in {}.", dir.underline().green()),
        bold
    );
    let f_parser = start_convert_and_parse(f)?;
    alert_cli!(format!("\u{1F4D1} | Parsing {}", "markdown".green()), bold);

    if let Some(true) = global_css {
        let c = HtmlWriter::add_link(f_parser);
        alert_cli!(format!("\u{1F534} | Adding global {}.", "CSS".green()), bold);
        let some = HtmlWriter::replace_markdown(c);
        let parsed = Component::replace(components, &some)?;
        alert_cli!(format!("\u{1F4C3} | Replacing {}.", "markdown".green()), bold);
        FolderBuilder::create_folder()?;
        FolderBuilder::start_creating_files(&parsed)?;
        return Ok(());
    }
    let some = HtmlWriter::replace_markdown(f_parser);
    let parsed = Component::replace(components, &some)?;
    alert_cli!(format!("\u{1F4C3} | Replacing {}.", "markdown".green()), bold);
    FolderBuilder::create_folder()?;
    FolderBuilder::start_creating_files(&parsed)?;
    Ok(())
}

pub fn process() -> Result<(), Error> {
    let conf = Config::read_config();
    if let Err(e) = conf {
        match e {
            Error::FileIOError(_) => {
                return Err(Error::ConfigFileError(
                    error::ConfigError::ConfigFileNotFound(
                        "Config file not found. Make sure you have a file named `resticular.toml`"
                            .to_string(),
                    ),
                ));
            }
            _ => return Err(e),
        }
    }
    let t_process = thread::spawn(move || -> Result<(), Error> {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        let config = Config::read_config()?;
        match config {
            Config {
                out_dir: _,
                source,
                lazy_images: _,
                routes: _,
                global_css: _,
                exclude: _,
                command: _
            } => {
                sub_process(&source)?;
                let eye_msg = MsgHandler::new();
                handle_thread_error_with_error!(
                    eye_msg.send(EyeKeeper::Unchanged),
                    Error::MsgError
                );
                let msg = MsgHandler::new();
                loop {
                    match msg.receive() {
                        Ok(ProcessIndicator::True) => {
                            sub_process(&source)?;
                        }
                        Ok(ProcessIndicator::False) => {
                            sub_process(&source)?;
                        }
                        Err(_) => {
                            todo!()
                        }
                    }
                }
                // Ok(())
            }
        }
    });

    let t_eye_keeper = thread::spawn(move || -> Result<(), Error> {
        loop {
            let msg = MsgHandler::new().receive();
            match msg {
                Ok(EyeKeeper::Changed) => {
                    println!("Changed");
                    // watch()?;
                    Ok(())
                }
                Err(_) => Err(Error::ThreadFailed(
                    "EYE_KEEPER_THREAD".to_owned(),
                    "keeping an eye and sending changes status".to_owned(),
                )),
                Ok(_) => {
                    // println!("{:?}", a);
                    continue;
                }
            };
        }
    });

    let t_fs_watcher = thread::spawn(|| -> Result<(), Error> {
        match watch() {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::ThreadFailed(
                "FS_WATCHER_THREAD".to_owned(),
                "watching the file system".to_owned(),
            )),
        }
    });

    let t_server = thread::spawn(|| -> Result<(), Error> {
        let rt = Runtime::new().expect("Error");
        rt.block_on(async move {
            alert_cli!(format!("Development server started on {} .", "http://localhost:3000".purple()), bold);
            match crate::core::http::server().await {
                Ok(_) => (),
                Err(er) => panic!("Server thread panicked: {}", er),
            }
        });
        Ok(())
    });

    let main_thread_handler = JoinHandler {
        t1: t_process,
        t2: t_eye_keeper,
        t3: t_fs_watcher,
        t4: t_server,
    };

    main_thread_handler.join();

    Ok(())
}
