use std::io::{stdin, stdout, stderr, Write};

use argparse::{ArgumentParser, Collect, IncrBy, DecrBy, StoreTrue};
use midir::{MidiInput};

use crate::utils::{get_best_matching_idx, PROGRAM_NAME, rewrap};

pub fn watch(args: Vec<String>) -> Result<(), (i32, String)> {
    let mut options = SetupOptions::default();
    let mut run_options = RunOptions::default();
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut options.devices).add_option(&["-d"], Collect, "add a device to monitor");
        ap.refer(&mut run_options.ignore).add_option(&["-i"], Collect, "add a message to ignore");
        ap.refer(&mut options.verbosity).add_option(&["-v"], IncrBy(1), "increase verbosity").add_option(&["-q"], DecrBy(1), "decrease verbosity");
        ap.refer(&mut run_options.timestamps).add_option(&["-t"], StoreTrue, "show timestamps");

        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(_) => {}
            Err(e) => return Err((e, String::new()))
        }
    }

    let midi_in = match MidiInput::new(PROGRAM_NAME) {
        Ok(input) => input,
        Err(e) => return Err((1, format!("error creating midi input")))
    };
    let ports = midi_in.ports();
    let mut connections = Vec::new();

    for d in &options.devices {
        match get_best_matching_idx(&midi_in, &ports, &d) {
            Ok(o) => {
                match o {
                    None => {},
                    Some(i) => {
                        connections.push(
                            rewrap(
                                rewrap(MidiInput::new(PROGRAM_NAME), options.verbosity, 1, "error creating midi input")?
                                .connect(
                                    &ports[i],
                                    PROGRAM_NAME,
                                    |stamp, msg, data| print_msg(stamp, msg, &data),
                                    run_options.clone()
                                ),
                                options.verbosity, 1, "error connecting to port"
                            )?
                        )
                    }
                }
            }
            Err(e) => return Err((1, format!("error getting port data")))
        }
    }

    println!("");

    // keep the main thread running
    let mut buf = String::new();
    println!("press enter to exit...");
    //let _ = stdout().flush();
    let _ = stdin().read_line(&mut buf);

    Ok(())
}

#[derive(Default)]
struct SetupOptions {
    pub devices: Vec<String>,
    pub verbosity: i32,
}
#[derive(Clone, Default)]
struct RunOptions {
    pub ignore: Vec<u8>,
    pub timestamps: bool
}

fn print_msg(stamp: u64, msg: &[u8], data: &RunOptions) {
    if !data.ignore.contains(&msg[0]) {
        let msg_type = msg[0] & 0xf0;
        let chan = (msg[0] & 0x0f) + 1;
        if data.timestamps {
            print!("{:016x}", stamp)
        }
        println!("{:02x} {:02x} {:02x} {:02}", msg_type, msg[1], msg[2], chan)
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
