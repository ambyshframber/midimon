use std::sync::{Arc, Mutex, mpsc:: Sender};

use super::FFCommand;

pub struct MessageTable {
    messages: Arc<Mutex<Vec<String>>>,
    msg_dirty: Arc<Mutex<bool>>,
    command_tx: Sender<FFCommand>
}
impl MessageTable {
    fn new(messages: Arc<Mutex<Vec<String>>>, msg_dirty: Arc<Mutex<bool>>, tx: Sender<FFCommand>) -> MessageTable {
        MessageTable {
            messages, msg_dirty,
            command_tx: tx
        }
    }
}

pub fn clear_text() {
    print!("\x1B[2J\x1B[1;1H");
}
/// 1,1 is top left
fn put_curs(x: i32, y: i32) {
    print!("\x1b[{};{}H", x, y)
}