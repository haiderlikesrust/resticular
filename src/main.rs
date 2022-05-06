

use resticular::error::Error;

use resticular::{process};
use resticular::core::fs::watcher::watch;
fn main() -> Result<(), Error> {
    process()?;
    Ok(())
}
