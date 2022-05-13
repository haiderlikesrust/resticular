

use std::thread;

use resticular::core::JoinHandler;
use resticular::error::Error;

use resticular::{process};
use resticular::core::fs::watcher::watch;
use tokio::runtime::Runtime;
use tracing::info;
fn main() -> Result<(), Error> {
   Ok(process()?)
}
