#[macro_use]
extern crate structopt;

use todo::*;
use termion::screen::AlternateScreen;
use std::io::stdout;
use std::io::Write;
use std::fmt::Error;
use std::result::Result::Ok;

#[derive(StructOpt, Debug)]
#[structopt(name = "todo")]
struct Opt {
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,
}

fn main() -> Result<(), Error>{
    match run() {
        Ok(s) => {
            println!("{}",s);
        },
        Err(_) => {},
    };


//    loop {
//        let screen = AlternateScreen::from(stdout());
//        screen.lock().flush();
//        match run(&mut config_options, screen) {
//            Some(_) => {
//                break
//            },
//            None => {
//            },
//        }
//    }
    Ok(())
}
