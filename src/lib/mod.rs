use crate::core::fs::watcher::watch;

use crate::core::http::MsgHandler;
use crate::core::JoinHandler;

use std::thread;

use crate::core::config::Config;
use crate::core::fs::reader::{FolderBuilder, Reader};
use crate::core::html::HtmlWriter;

use error::Error;
use tokio::runtime::Runtime;
/// Module on which resticular's functionality depends.
pub mod core;
pub mod error;
pub mod prelude;
use crate::core::fs::reader::start_convert_and_parse;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
    let reader = Reader::new(dir.into());
    info!("Creating file reader..");
    let f = reader.read_dir_files()?;
    info!("Reading {}", dir);
    let f_parser = start_convert_and_parse(f);
    info!("Parsing markdown.");
    let c = HtmlWriter::add_link(f_parser);
    info!("Adding css");
    let some = HtmlWriter::replace_markdown(c);
    info!("Replacing markdown");
    FolderBuilder::create_folder()?;
    FolderBuilder::start_creating_files(&some)?;
    Ok(())
}

pub fn process() -> Result<(), Error> {
    let conf = Config::read_config();
    if let Err(e) = conf {
        return Err(e);
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
                dir,
                lazy_images: _,
            } => {
                sub_process(&dir)?;
                let eye_msg = MsgHandler::new();
                eye_msg.send(EyeKeeper::Unchanged);
                let msg = MsgHandler::new();
                loop {
                    match msg.receive() {
                        Ok(ProcessIndicator::True) => {
                            sub_process(&dir)?;
                        }
                        Ok(ProcessIndicator::False) => {
                            sub_process(&dir)?;
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
            let msg = MsgHandler::new();
            match msg.receive() {
                Ok(EyeKeeper::Changed) => {
                    println!("Changed");
                    // watch()?;
                    Ok(())
                }
                Err(_) => Err(Error::ThreadFailed(
                    "EYE_KEEPER_THREAD".to_owned(),
                    "keeping an eye and sending changes status".to_owned(),
                )),
                Ok(a) => {
                    println!("{:?}", a);
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
            info!("Development server started on http://localhost:3000");
            crate::core::http::server().await;
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
