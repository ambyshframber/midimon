use std::error::Error;

use argparse::{ArgumentParser, Collect};
use midir::{MidiInput};

use crate::utils::get_best_matching_idx;

pub fn watch() -> Result<(), Box<dyn Error>> {
    let mut options = SetupOptions::default();
    let mut run_options = RunOptions::default();
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut options.devices).add_option(&["-d"], Collect, "add a device to monitor");
        ap.refer(&mut run_options.ignore).add_option(&["-i"], Collect, "add a message to ignore");

        ap.parse_args_or_exit()
    }

    let midi_in = MidiInput::new("midimon")?;
    let ports = midi_in.ports();
    let mut connections = Vec::new();

    for d in &options.devices {
        match get_best_matching_idx(&midi_in, &ports, &d)? {
            None => {},
            Some(i) => {
                connections.push(MidiInput::new("midimon")?.connect(&ports[i], "midimon", |stamp, msg, data| print_msg(stamp, msg, data), run_options.clone())?);
            }
        }
    }

    println!("");

    loop {} // keep the main thread running
}

#[derive(Default)]
struct SetupOptions {
    pub devices: Vec<String>
}
#[derive(Clone, Default)]
struct RunOptions {
    pub ignore: Vec<u8>
}

fn print_msg(stamp: u64, msg: &[u8], data: &RunOptions) {
    if !data.ignore.contains(&msg[0]) {
        let msg_type = msg[0] & 0xf0;
        let chan = (msg[0] & 0x0f) + 1;
        println!("{:016x} {:02x} {:02x} {:02x} {:02}", stamp, msg_type, msg[1], msg[2], chan)
    }
}

fn msg_to_description(msg: &[u8]) -> &'static str {
    assert!(msg.len() != 0); // IF THIS HAPPENS YOU HAVE ISSUES

    match msg[0] {
        0b1000_0000 => "NOTE OFF",
        0b1001_0000 => "NOTE ON",
        0b1010_0000 => "POLY KEY PRESSURE",
        0b1011_0000 => { // control change / channel mode
            "CONTROL CHANGE"
        }
        0b1100_0000 => "PROGRAM CHANGE",
        0b1101_0000 => "CHANNEL PRESSURE",
        0b1110_0000 => "PITCH BEND",

        _ => "UNDEFINED" // system common/realtime (FIX)
    }
}
