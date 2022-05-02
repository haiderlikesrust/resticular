use crate::core::fs::reader::Reader;
use crate::core::config::Config;
use crate::core::html::HtmlWriter;

use error::Error;

/// Module on which resticular's functionality depends.
pub mod core;
pub mod error;
pub mod prelude;
use crate::core::fs::reader::start_convert_and_parse;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
pub fn process() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    let config = Config::read_config()?;
    match config {
        Config {
            out_dir: _,
            dir,
            lazy_images: _,
        } => {
            let reader = Reader::new(dir.clone().into());
            info!("Creating file reader..");
            let f = reader.read_dir_files()?;
            info!("Reading {}", dir);
            let f_parser = start_convert_and_parse(f);
            info!("Parsing markdown.");
            let c = HtmlWriter::add_link(f_parser);
            info!("Adding css");
            let some = HtmlWriter::replace_markdown(c);
            info!("Replacing markdown");
            println!("{:#?}", some)
        }
    }
    Ok(())
}
