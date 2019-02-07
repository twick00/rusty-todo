#![feature(fn_traits)]
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
use std::borrow::Cow;
use std::process::Output;
use std::slice::Chunks;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::BufReader;
use std::fmt::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigOptions {
    pub lines_per_tab: usize,
    pub tickets: Vec<String>,
    pub jira_query: String
}

impl ConfigOptions {
    fn default() -> Self {
        ConfigOptions {
            lines_per_tab: 5,
            tickets: vec![],
            jira_query: String::from("project = \"CATS\" ORDER BY priority DESC")
        }
    }
    pub fn from_default_config() -> Self {
        let mut file = get_config_file();
        let reader = BufReader::new(file);

        //get or create new config file and return it
        let mut config_file = match serde_json::from_reader(reader) {
            Ok(cfg_file) => {
                cfg_file
            },
            Err(_) => {
                ConfigOptions::default().save_config()
            },
        };
        config_file.fetch_tickets_into_self();
        config_file.save_config()
    }

    fn save_config(self) -> Self {
        let serialized = serde_json::to_string_pretty(&self).expect("Failed to serialize self.");
        get_config_file().write(serialized.as_ref());
        self
    }

    fn fetch_tickets_into_self(&mut self) {
        let mut tickets = get_todo_list();
        tickets.split_off(self.lines_per_tab);
        self.tickets = tickets;
    }
}

pub fn run() -> Result<String, Error> {
    let mut cfg_options = ConfigOptions::from_default_config();
    let ticket_vec = cfg_options.tickets.to_vec();
    if ticket_vec.len() < 1 {
        return Ok(String::from("No Tickets found"))
    }
    loop {
        match select_ticket(ticket_vec.to_owned()) {
            _ => {}
        }
    }
}

fn get_config_file() -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("config.json")
        .expect("Failed to find, open or create config.json")
}

pub fn select_ticket(list: Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which ticket?")
        .default(0)
        .items(&list)
        .interact()
        .expect("Expected a parsable input.")
}

pub fn select_options(select_ticket: usize) -> (usize, usize) {
    let options = vec!["View Details", "Go To Ticket", "Exit", "Back"];
    let option = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();
    (select_ticket, option)
}

pub fn get_todo_list() -> Vec<String> {
    let output: Output = Command::new("jira")
        .arg("list")
        .arg("--query")
        .arg("project = \"CATS\" ORDER BY priority DESC")
        .output()
        .expect("failed to execute process");
    let read_in_stream = String::from_utf8_lossy(output.stdout.as_ref());
    read_in_stream.lines().into_iter().map(|line| line.to_owned()).collect()
}


