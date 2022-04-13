use std::env::args;
use std::process::exit;

use watch::watch;
use list::list;

mod watch;
mod list;
mod utils;
mod circular_buf;

fn main() {
    match run() {
        Ok(_) => (),
        Err((e, s)) => {
            println!("{}", s);
            exit(e)
        }
    }
}

fn run() -> Result<(), (i32, String)> {
    let mut args = args();
    args.next();
    let args: Vec<String> = args.collect();
    if args.len() == 0 {
        return Err((2, String::from("subcommand required")))
    }
    match args[0].as_str() {
        "watch" => {
            watch(args)
        }
        "list" => {
            list(args)
        }
        _ => {
            Err((2, format!("unknown subcommand: {}", args[0])))
        }
    }
}