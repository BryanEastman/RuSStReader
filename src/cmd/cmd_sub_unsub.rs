use reqwest::blocking::Client;
use std::fs::File;

pub fn subscribe(
    current_subscriptions: &Vec<String>,
    client: Client,
    new_subscriptions: Vec<String>,
    file: File,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Parsing subscriptions");
    let mut csv_writer = csv::Writer::from_writer(file);
    for item in new_subscriptions {
        // validate url before writing
        match client.get(&item).send() {
            Ok(req) => match req.error_for_status() {
                Ok(response) => {
                    println!("Site is valid: {}", response.status().as_str());
                    // check existing subscriptions before writing
                    if !current_subscriptions.contains(&item) {
                        println!("Subscribing to {item}");
                        csv_writer.write_record([item])?;
                    } else {
                        println!("already subscribed to {item}, skipping");
                        continue;
                    }
                }
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            },
            Err(e) => {
                println!("{e} Url malformed for item: {item}");
                continue;
            }
        }
    }
    csv_writer.flush().unwrap();
    Ok(())
}
