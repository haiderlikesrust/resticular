use std::time::Duration;

use hotwatch::{
    blocking::{Flow, Hotwatch},
    Event,
};

use crate::{
    core::{config::Config, http::MsgHandler},
    error::Error,
};

pub fn watch() -> Result<(), Error> {
    let config = Config::read_config()?;
    let mut hotwatch =
        Hotwatch::new_with_custom_delay(Duration::from_secs(1)).expect("Error hotwatch");
    hotwatch
        .watch(config.dir, |event: Event| {
            let msg = MsgHandler::new();
            match event {
                Event::Create(e) => {
                    println!("New: {}", e.to_str().unwrap());
                    msg.send(crate::EyeKeeper::Changed);
                    Flow::Continue
                }
                Event::Write(e) => {
                    println!("changed: {}", e.to_str().unwrap());
                    msg.send(crate::EyeKeeper::Changed);
                    Flow::Continue
                }
                _ => Flow::Continue,
            }
        })
        .expect("failed to watch file!");
    hotwatch.run();
    Ok(())
}