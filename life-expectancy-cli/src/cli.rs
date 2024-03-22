use clap::{Parser, Subcommand};

/// Health Data Query CLI
#[derive(Parser, Debug)]
#[clap(author = "Naveed Iqbal", version = "1.0.0", about = "This utility cli is to query health data by specific country and given indicator", long_about = None)]
pub struct Cli {
    /// Path to a custom CSV dataset
    #[clap(short, long, value_parser, default_value = "./dataset/data.csv")]
    pub file: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Executes a query against the dataset
    Query {
        /// The country to query data for
        #[clap(short, long, value_parser)]
        country: String,

        /// The year to query data for. If omitted, returns data for all available years
        #[clap(short, long, value_parser)]
        year: Option<u32>,

        /// Health indicator to query (life_expectancy, adult_mortality, infant_deaths, alcohol, polio)
        #[clap(short, long, value_parser)]
        indicator: String,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
