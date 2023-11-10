use crate::importer::{import_edges, import_nodes};
use crate::models::{config::Config, csv::Edge, csv::Node};
use clap::{App, Arg, SubCommand};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::{println, process};

mod importer;
mod models;

#[tokio::main]
async fn main() {
    let matches = App::new("CSV to Gremlin importer")
        .version("1.0")
        .author("Davi Suga")
        .about("Imports CSV data into Gremlin graph database")
        .subcommand(
            SubCommand::with_name("nodes")
                .about("Imports CSV data as nodes")
                .arg(
                    Arg::with_name("config")
                        .help("Sets the config file to use")
                        .required(true),
                )
                .arg(
                    Arg::with_name("csv")
                        .help("Sets the CSV file to import")
                        .required(true),
                )
                .arg(
                    Arg::with_name("headers")
                        .help("CSV headers (comma-separated)")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("edges")
                .about("Imports CSV data as edges")
                .arg(
                    Arg::with_name("config")
                        .help("Sets the config file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("csv")
                        .help("Sets the CSV file to import")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("nodes") {
        let config_file = sub_matches.value_of("config").unwrap();
        let config: Config = read_config(config_file).unwrap();
        println!("Importing nodes");
        let file_path = sub_matches.value_of("csv").unwrap();
        println!("Reading CSV file from {}", file_path);
        let headers = sub_matches.value_of("headers");
        println!("Headers: {:?}", headers);
        let csv_lines: Vec<HashMap<String, String>> = read_csv(file_path, headers).unwrap();
        let nodes = csv_lines
            .iter()
            .map(|line| Node {
                properties: line.clone(),
                label: "person".to_owned(),
            })
            .collect::<Vec<_>>();
        import_nodes(&config, &nodes).await.unwrap();
    } else if let Some(sub_matches) = matches.subcommand_matches("edges") {
        let config_file = sub_matches.value_of("config").unwrap();
        let config: Config = read_config(config_file).unwrap();
        let file_path = sub_matches.value_of("csv").unwrap();
        let headers = sub_matches.value_of("headers");

        let csv_lines: Vec<HashMap<String, String>> = read_csv(file_path, headers).unwrap();
        let edges = csv_lines
            .iter()
            .map(|line| Edge {
                from: line.get("from").unwrap().to_owned(),
                to: line.get("to").unwrap().to_owned(),
                relationship: line.get("relationship").unwrap().to_owned(),
            })
            .collect::<Vec<_>>();
        import_edges(&config, &edges).await.unwrap();
    }
    ()
}

fn read_config<T: for<'de> Deserialize<'de>>(file_path: &str) -> Result<T, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let t = serde_yaml::from_reader(reader)?;
    Ok(t)
}

fn read_csv<T: for<'de> Deserialize<'de> + std::fmt::Debug>(
    file_path: &str,
    headers: Option<&str>,
) -> Result<Vec<T>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(headers.is_none())
        .from_path(file_path)?;
    if let Some(h) = headers {
        rdr.set_headers(csv::StringRecord::from(h.split(',').collect::<Vec<_>>()));
    }

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record)
    }
    Ok(records)
}
