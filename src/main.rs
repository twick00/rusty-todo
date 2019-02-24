#[macro_use]
extern crate serde_derive;

use config::Config;
use config::ConfigError;
use std::process::Command;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs::File;
use std::io::Write;


fn main() {
    match run() {
        None => {
            println!("An Error Occurred")
        }
        Some(exit_message) => {
            print!("{}", exit_message)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DefaultConfig {
    pub jira_query: String,
}

impl DefaultConfig {
    pub fn new() -> Self {
        DefaultConfig {
            jira_query: String::from("assignee=currentUser() AND status in ('Ready For Dev', 'In Dev') order by priority")
        }
    }
}

fn run() -> Option<String> {
    let config: DefaultConfig = match configure() {
        Ok(config) => {
            config
        }
        Err(error) => {
            return Some(error.to_string());
        }
    };
    let tickets = match get_tickets(&config) {
        Some(tickets) => {
            tickets
        }
        None => {
            return Some(String::from("\nNo tickets found"));
        }
    };

    loop {
        let selected_ticket = match select_ticket(&tickets) {
            Some(selected_ticket) => {
                if selected_ticket >= tickets.len() {
                    return Some(String::from(""));
                }
                tickets.get(selected_ticket).cloned()
            }
            None => {
                panic!("An error ocurred when parsing selected ticket.")
            }
        }.unwrap();
        match select_options() {
            //Go To Ticket
            Some(0) => {
                go_to_ticket(&selected_ticket);
                break;
            }
            //Exit
            Some(1) => {
                break;
            }
            //Back
            Some(2) => {}
            _ => {
                return Some(String::from("An error occurred when parsing option selection"));
            }
        }
    }

    Some(String::from(""))
}

fn go_to_ticket(ticket: &String) {
    let slice: Vec<&str> = ticket.split(":").collect::<Vec<&str>>();

    Command::new("jira")
        .arg("browse")
        .arg(slice.first().unwrap())
        .output()
        .expect("failed to execute process");
}

fn select_options() -> Option<usize> {
    let options = vec!["Go To Ticket", "Exit", "Back"];
    Some(Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .items(&options)
        .interact()
        .unwrap())
}

fn select_ticket(tickets: &Vec<String>) -> Option<usize> {
    Some(Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which ticket?")
        .default(0)
        .items(&tickets)
        .item("Exit")
        .interact().unwrap())
}

fn get_tickets(config: &DefaultConfig) -> Option<Vec<String>> {
    let jira_query: String = config.jira_query.clone();
    let output = Command::new("jira")
        .arg("list")
        .arg("--query")
        .arg(jira_query)
        .output()
        .expect("failed to execute jira cli.. perhaps you haven't installed go-jira?");
    let read_in_stream = String::from_utf8_lossy(output.stdout.as_ref());
    let tickets: Vec<String> = read_in_stream.lines().into_iter().map(|line| line.to_owned()).collect();
    match tickets.len() {
        size if size >= 1 => {
            Some(tickets)
        }
        size if size <= 0 => {
            //No tickets
            None
        }
        _ => {
            //Error
            panic!("Failed to fetch tickets.")
        }
    }
}


fn configure() -> Result<DefaultConfig, ConfigError> {
    let mut config = config::Config::default();
    match config.merge(config::File::with_name("todo_config.json")) {
        Ok(config) => {
            config
        }
        Err(_) => {
            let new_config = create_new_config();
            return Ok(new_config.to_owned().deserialize().unwrap());
        }
    };
    Ok(config.deserialize().unwrap())
}

fn create_new_config() -> Config {
    let new_config = DefaultConfig::new();
    let config_string = match serde_json::to_string_pretty(&new_config) {
        Ok(config) => {
            config
        }
        Err(_) => {
            panic!("Failed to serialize new default config into string")
        }
    };
    let mut file = File::create("todo_config.json").unwrap();
    file.set_len(0);
    file.write(config_string.as_ref());

    config::Config::default().merge(config::File::with_name("todo_config.json")).unwrap().to_owned()
}
