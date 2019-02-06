#[macro_use]
extern crate structopt;

use std::env;

use todo::*;
use std::fmt::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "todo")]
struct Opt {
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,
}

fn main() -> Option<String> {
    let list = get_todo_list();
    if list.len() < 1 {
        println!("No Tickets Found");
        return None
    }
    loop {
        match run(list) {
            Some(_) => {
                break
            },
            None => {},
        }
    }
}
