
use crate::core::IntoInner;

use crate::core::fs::reader::Reader;

use crate::core::config::Config;
use crate::core::html::HtmlWriter;

use error::Error;

/// Module on which resticular's functionality depends.
pub mod core;
pub mod error;
pub mod prelude;
use crate::core::fs::reader::start_convert_and_parse;

pub fn process() -> Result<(), Error> {
    let config = Config::read_config()?;
    match config {
        Config {
            out_dir: _,
            dir,
            lazy_images: _,
        } => {
            let reader = Reader::new(dir.into());
            let f = reader.read_dir_files()?;
            let f_parser = start_convert_and_parse(f);
            let c = HtmlWriter::add_link(f_parser);
            for page in c {
                println!("{:#?}", page.content.into_inner());
            }

        }
    }
    Ok(())
}

