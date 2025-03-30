use clap::{Parser, Subcommand};
use csv::StringRecord;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;

use cmd::cmd_sub_unsub;
mod article_parser;
mod cmd;
const SUBSCRIPTIONS_PATH: &str = "./data/subscriptions.csv";

#[derive(Parser, Debug)]
#[command(version, about, long_about=None, next_line_help=true)]
struct Cli {
    // URL of feed to subscribe to
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Subscribe {
        new_subscriptions: Vec<String>,
    },

    // URL of feed to subscribe to
    #[command(arg_required_else_help = true)]
    Unsubscribe {
        unsub_from: String,
    },

    // Refreshes articles for subscribed feeds
    Refresh,

    // Test command
    #[command(arg_required_else_help = true)]
    Test {
        test_val: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Subscriptions {
    urls: Vec<String>,
}

fn start() -> Result<Client, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .expect("Failed to create client");

    Ok(client)
}

fn fetch_current_articles(mut reader: csv::Reader<&File>) -> Result<Vec<String>, Box<dyn Error>> {
    let current_subscriptions: Vec<String> = reader
        .records()
        .filter_map(|result| {
            result.ok().and_then(|record| {
                if record.is_empty() {
                    None
                } else {
                    Some(record[0].to_string())
                }
            })
        })
        .collect();
    println!("Current Subscriptions: ");
    // for item in &current_subscriptions {
    //     print!("{:?}", &item);
    // }

    Ok(current_subscriptions)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(SUBSCRIPTIONS_PATH)
        .unwrap();

    let client = start().unwrap();
    let mut csv_reader = csv::Reader::from_reader(&file);
    csv_reader.set_headers(StringRecord::from(vec!["articles"]));

    let current_subscriptions: Vec<String> = fetch_current_articles(csv_reader)?;

    let args = Cli::parse();

    match args.command {
        Commands::Subscribe { new_subscriptions } => {
            cmd_sub_unsub::subscribe(&current_subscriptions, client, new_subscriptions, file)?;
        }
        Commands::Unsubscribe { unsub_from } => {
            println!("Unsubscribing from {unsub_from}");
        }
        Commands::Test { test_val } => {
            println!("Test Value: {test_val}");
            println!("{:?}", current_subscriptions);
        }
        Commands::Refresh => {
            println!("Refreshing feed");
            let articles = article_parser::parse_articles(&current_subscriptions, &client)?;
            for channel in articles {
                println!("{}\n", &channel.title);
                for article in channel.articles {
                    println!("{article}\n");
                }
            }
        }
    }
    Ok(())
}
