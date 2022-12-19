extern crate tokio;
use std::env;

mod get_stats;
use get_stats::{fetch, print_stats};
mod get_sirens;
mod show_glory;
use show_glory::trident;
enum Command {
    Trident,
    Stats,
    Sirens,
    Help,
    Other,
}
struct Args {
    command: String,
}
impl Args {
    pub fn new(args: &[String]) -> Result<Args, &str> {
        if args.len() < 2 {
            return Err("?!");
        }
        let command = args[1].clone();
        Ok(Args { command })
    }
}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Args::new(&args)
        .unwrap_or_else(|_err| Args {
            command: String::from(""),
        })
        .command;
    println!("{}", command);
    let action;
    if command == "trident" {
        action = Command::Trident;
    } else if command == "stats" {
        action = Command::Stats;
    } else if command == "help" {
        action = Command::Help;
    } else if command == "siren" {
        action = Command::Sirens;
    } else {
        action = Command::Other;
    } //I would like to do it without else if
    match action {
        Command::Trident => trident(),
        Command::Stats => {
            let response = fetch().await;
            match response {
                Ok(data) => {
                    let actual_data = data.data;
                    let (table, day) = print_stats(actual_data);
                    println!(
                        "The losses of the fascist army on the {} day of the war amounted to:",
                        day
                    );
                    println!("{}", table);
                }
                Err(err) => println!("{}", err),
            }
        }
        Command::Sirens => {
            let response = get_sirens::fetch().await;
            match response {
                Ok(data) => {
                    //let actual_data = data.states;
                    get_sirens::print_sirens(get_sirens::filter_regions(data));
                }
                Err(err) => println!("{}", err),
            }
        }
        Command::Help => {
            println!("{0: <10} {1: <10}", "command", "action");
            println!(
                "{0: <10} {1: <10}",
            "stats",  "Display the up-to-date data on the Russian invasion of Ukraine according to https://russianwarship.rip/");
            println!(
                "{0: <10} {1: <10}",
                "siren", "List the areas in which there is now an air raid alert. Using https://vadymklymenko.com/map/"
            );
            println!(
                "{0: <10} {1: <10}",
                "trident", "Draw the National Emblem of Ukraine."
            );
            println!("Glory to Ukraine")
        }
        Command::Other => {
            println!("Unknown argument. You can use 'help' command for more information.")
        }
    }
}
