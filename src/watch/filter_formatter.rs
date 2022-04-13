use std::sync::{Arc, Mutex, mpsc::{channel, Receiver, Sender}};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiInput, MidiInputConnection};

use crate::circular_buf::CircularBuf;
use crate::utils::MidiMessage;
use super::{FFCommand, Ignore};

pub struct FilterFormatter {
    pub messages: Arc<Mutex<Vec<String>>>, // send to msg table
    pub msg_dirty: Arc<Mutex<bool>>,
    internal_msg_dirty: bool,
    connections: Vec<MidiInputConnection<Sender<MidiMessage>>>,
    buf: CircularBuf<MidiMessage>,
    midi_rx: Receiver<MidiMessage>,
    midi_tx: Sender<MidiMessage>,
    command_rx: Receiver<FFCommand>,
    filter: FilterSettings
}
impl FilterFormatter {
    pub fn new(buffer_size: usize, rx: Receiver<FFCommand>) -> FilterFormatter {
        let (mtx, mrx) = channel();
        FilterFormatter {
            messages: Arc::new(Mutex::new(Vec::new())),
            msg_dirty: Arc::new(Mutex::new(false)),
            internal_msg_dirty: false,
            connections: Vec::new(),
            buf: CircularBuf::new(buffer_size),
            midi_rx: mrx,
            midi_tx: mtx,
            command_rx: rx,
            filter: FilterSettings::default()
        }
    }
    pub fn main_loop(&mut self) {
        loop {
            for msg in self.midi_rx.try_iter() { // get all incoming messages
                if msg.1[0] != 248 { // lazy filter for now
                    dbg!(msg);
                    self.buf.push(msg);
                    self.internal_msg_dirty = true;
                }
            }
            
            let mut commands = Vec::new();
            // format midi msgs here
            for msg in self.command_rx.try_iter() { // get all pending commands
                commands.push(msg)
            }
            for c in commands {
                self.do_command(c)
            }

            {
                let mut dirty = self.msg_dirty.lock().unwrap();
                *dirty = self.internal_msg_dirty
            }

            sleep(Duration::from_millis(1))
        }
    }
    fn do_command(&mut self, command: FFCommand) {
        match command {
            FFCommand::List => {
                println!("listing midi devices...")
            }
            _ => {}
        }
        self.internal_msg_dirty = true;
    }
}

#[derive(Default)]
pub struct FilterSettings {

}


fn send_msg(stamp: u64, msg: &[u8], data: Sender<MidiMessage>) {
    let mut bytes = [0; 3];
    for (i, b) in msg.iter().enumerate() {
        bytes[i] = *b // won't panic unless you have midi demons
    }
    let _ = data.send((stamp, bytes));
}
fn msg_to_description(msg: &[u8]) -> &str {
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
