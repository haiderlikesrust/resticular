use std::time::Duration;

use crate::error::Error;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    Event,
};

use crate::{
    core::{
        config::Config,
        http::{ws::WsHandler, MsgHandler},
    },
    handle_thread_error, handle_thread_error_with_error, sub_process, ProcessIndicator,
};

pub fn watch() -> Result<(), Error> {
    let config = Config::read_config()?;
    let mut hotwatch =
        Hotwatch::new_with_custom_delay(Duration::from_secs(1)).expect("Error hotwatch");
    hotwatch.watch(config.dir, |event: Event| {
        let conf = Config::read_config().unwrap();
        let msg = MsgHandler::new();
        let ws = WsHandler::new("http://0.0.0.0:4200/");
        match event {
            Event::Create(e) => {
                println!("New: {}", e.to_str().unwrap());
                handle_thread_error_with_error!(
                    msg.send(crate::EyeKeeper::Changed),
                    Error::MsgError
                );
                handle_thread_error!(sub_process(&conf.dir));
                ws.out();
                Flow::Continue
            }
            Event::Write(e) => {
                println!("changed: {}", e.to_str().unwrap());

                handle_thread_error_with_error!(
                    msg.send(crate::EyeKeeper::Changed),
                    Error::MsgError
                );
                handle_thread_error!(sub_process(&conf.dir));
                println!("BOOM");
                Flow::Continue
            }
            _ => Flow::Continue,
        }
    })?;
    hotwatch.run();
    Ok(())
}
