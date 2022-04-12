use std::error::Error;

use watch::watch;

mod watch;
mod utils;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    watch()?;

    Ok(())
}