#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;
use std::io::{Stdout, Write, stdout};
use std::io;
use termion::screen::AlternateScreen;
use std::thread;
use std::time;
use std::ops::Deref;
use std::io::StdoutLock;
use std::slice::Split;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    ticket_options: Vec<Option>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Option<'a> {
    heading: &'a str,
    runnable: closure,
}

pub fn select_ticket(list: &Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which ticket?")
        .default(0)
        .items(&list)
        .interact()
        .expect("Expected a parsable input.")
}

pub fn select_options(select_ticket: usize) -> (usize, usize) {
    let options = vec![
        "View Details", "Go To Ticket", "Back"
    ];

    let option = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();
    (select_ticket, option)
}

pub fn get_todo_list() -> Vec<String> {
//    let output = Command::new("jira")
//        .arg("list")
//        .arg("--query")
//        .arg("assignee=patrick.berke order by priority")
//        .output()
//        .expect("failed to execute process");
//    let mut input = String::from_utf8(output.stdout).unwrap();
    return test_data()
}

pub fn test_data() -> Vec<String> {
    vec![
        String::from("CATS-7581:   CLONE - This is a test"),
        String::from("CATS-7582:   CLONE - This is a test 2"),
        String::from("CATS-7583:   CLONE - This is a test 3"),
        String::from("CATS-7584:   CLONE - This is a test 4"),
        String::from("CATS-7585:   CLONE - This is a test 5"),
        String::from("CATS-7586:   CLONE - This is a test 6"),
    ]
}
pub fn run_interactive<'a>(list:Vec<String>, stdout_lock: StdoutLock) -> Option<&'a str> {
    if list.len() > 0 {
        let selected_ticket = select_ticket(&list);
        let option = select_options(selected_ticket);
        let ticket = list[selected_ticket].as_ref();

        match option {
            (_, 0) => {
                Some(ticket)
            },
            (_, 1) => {
                Command::new("issue").arg(format!("{}", ticket)).output();
                let split_ticket: Vec<&str> = ticket.split(":").into_iter().collect();
                Some(ticket)
            }
            _ => {
                None
            }
        }
    } else {
        Some("No Tickets Found")
    }
}

pub fn run(list: Vec<String>) -> Option<&'static str> {
    let mut screen = AlternateScreen::from(stdout());
    screen.flush().unwrap();
    match run_interactive(list, screen.lock()) {
        None => None,
        Some(s) => {
            write!(screen, "{}", s).unwrap();
            Some(s)
        }
    }
}
