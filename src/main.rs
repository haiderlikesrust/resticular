use std::fs;

use resticular::cli::Cli;
use resticular::core::config::Config;
use resticular::error::Error;
use resticular::process;
use std::io::Write;
use std::env::current_dir;
fn main() -> Result<(), Error> {
    
    Cli.start()?;
    Ok(())
    // Ok(process()?)
}
