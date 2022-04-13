use std::io::{stdout, stderr};
use std::sync::mpsc::channel;
use std::thread::{spawn};

use argparse::{ArgumentParser, Collect, IncrBy, DecrBy};

use filter_formatter::FilterFormatter;

mod msg_table;
mod filter_formatter;

pub fn watch(args: Vec<String>) -> Result<(), (i32, String)> {
    let mut options = SetupOptions::default();
    let mut ignore: Vec<String> = Vec::new();
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut options.devices).add_option(&["-d"], Collect, "add a device to monitor");
        ap.refer(&mut ignore).add_option(&["-i"], Collect, "add a message to ignore");
        ap.refer(&mut options.verbosity).add_option(&["-v"], IncrBy(1), "increase verbosity").add_option(&["-q"], DecrBy(1), "decrease verbosity");

        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(_) => {}
            Err(e) => return Err((e, String::new()))
        }
    }

    let (tx, rx) = channel::<FFCommand>();

    let mut filt = FilterFormatter::new(100, rx);
    let filt_thread = spawn(move || {
        filt.main_loop()
    });

    tx.send(FFCommand::List);

    filt_thread.join().unwrap();

    Ok(())
}

#[derive(Default)]
struct SetupOptions {
    pub devices: Vec<String>,
    pub verbosity: i32,
}
pub enum FFCommand {
    Connect(String),
    Disconnect(usize),
    List
}
#[derive(Default)]
pub struct Ignore {
    masks: [u8; 3],
    bits: [u8; 3 ]
}