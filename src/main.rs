use resticular::cli::Cli;

use resticular::error::Error;

fn main() -> Result<(), Error> {
    Cli.start()?;
    Ok(())
}
