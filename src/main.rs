use crate::importer::{import_edges, import_nodes};
use crate::models::{config::Config, csv::Edge, csv::Node};
use clap::{App, Arg, SubCommand};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process;

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
                    Arg::with_name("CONFIG")
                        .help("Sets the config file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("CSV")
                        .help("Sets the CSV file to import")
                        .required(true)
                        .index(2),
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
                    Arg::with_name("CONFIG")
                        .help("Sets the config file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("CSV")
                        .help("Sets the CSV file to import")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap();
    let config: Config = read_config(config_file).unwrap();

    if let Some(sub_matches) = matches.subcommand_matches("nodes") {
        let file_path = sub_matches.value_of("file").unwrap();
        let headers = sub_matches.value_of("headers");
        let nodes: Vec<Node> = read_csv(file_path, headers).unwrap();
        import_nodes(&config, &nodes).await.unwrap();
    } else if let Some(sub_matches) = matches.subcommand_matches("edges") {
        let file_path = sub_matches.value_of("file").unwrap();
        let headers = sub_matches.value_of("headers");

        let edges: Vec<Edge> = read_csv(file_path, headers).unwrap();
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

fn read_csv<T: for<'de> Deserialize<'de>>(
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
