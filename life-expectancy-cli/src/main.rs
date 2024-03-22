mod cli;
mod core;

use cli::{parse_args, Cli, Commands};
use core::*;

const DATA_URL: &str = "https://raw.githubusercontent.com/ageron/handson-ml2/master/datasets/lifesat/oecd_bli_2015.csv";
const DATA_FILE: &str = "./dataset/data.csv";

fn main() {
    let args: Cli = parse_args();
    let data = fetch_data(&args.file.unwrap());
    match args.command {
        Commands::Query {
            country,
            year,
            indicator,
        } => {
            let result = match indicator.as_str() {
                "life_expectancy" => calculate_life_expectancy(&data, &country, year.unwrap_or(0)),
                _ => panic!("Invalid indicator"),
            };
            println!("{}: {}", indicator, result);
        }
    }
}
