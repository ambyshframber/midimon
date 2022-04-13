use std::io::{stdout, stderr};

use midir::{MidiInput, MidiOutput};
use argparse::{ArgumentParser, DecrBy, IncrBy};

use crate::utils::{PROGRAM_NAME, rewrap};

pub fn list(args: Vec<String>) -> Result<(), (i32, String)> {
    let mut verbosity = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut verbosity).add_option(&["-v"], IncrBy(1), "increase verbosity").add_option(&["-q"], DecrBy(1), "decrease verbosity");
        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(_) => {}
            Err(e) => return Err((e, String::new()))
        }
    }

    let midi_in = rewrap(MidiInput::new(PROGRAM_NAME), verbosity, 1, "error creating midi input")?;
    let midi_out = rewrap(MidiOutput::new(PROGRAM_NAME), verbosity, 1, "error creating midi output")?;

    println!("output ports:");
    for (i, p) in midi_in.ports().iter().enumerate() {
        println!("\t{}: {}", i, rewrap(midi_in.port_name(p), verbosity, 1, "error getting port name")?)
    }
    println!("input ports:");
    for (i, p) in midi_out.ports().iter().enumerate() {
        println!("\t{}: {}", i, rewrap(midi_out.port_name(p), verbosity, 1, "error getting port name")?)
    }

    Ok(())
}
