// mod cli;
// mod core;

// use cli::{parse_args, Cli, Commands};
// use core::*;

// const DATA_URL: &str = "https://raw.githubusercontent.com/ageron/handson-ml2/master/datasets/lifesat/oecd_bli_2015.csv";
// const DATA_FILE: &str = "./dataset/data.csv";

// fn main() {
//     let args: Cli = parse_args();
//     let data = fetch_data(&args.file.unwrap());
//     match args.command {
//         Commands::Query {
//             country,
//             year,
//             indicator,
//         } => {
//             let result = match indicator.as_str() {
//                 "life_expectancy" => calculate_life_expectancy(&data, &country, year.unwrap_or(0)),
//                 _ => panic!("Invalid indicator"),
//             };
//             println!("{}: {}", indicator, result);
//         }
//     }
// }

// ETL Example

#[derive(Debug)]
struct RawData {
    id: u32,
    value: i32,
}

#[derive(Debug)]
struct CleanData {
    id: u32,
    value: i32,
}

fn main() {
    let raw = vec![
        RawData { id: 1, value: 10 },
        RawData { id: 2, value: -5 },
        RawData { id: 3, value: 100 },
        RawData { id: 4, value: 111 },
        RawData { id: 5, value: 19 },

    ];

    let cleaned = extract_transform_load(raw); 

    println!("Clean Data: Id - {:?} Value - {:?}", cleaned[0].id, cleaned[0].value); // Accessing the fields
}

// Perform ETL process 
fn extract_transform_load(raw: Vec<RawData>) -> Vec<CleanData> {
    let mut data: Vec<CleanData> = raw.into_iter() 
        .map(|r| CleanData { 
            id: r.id,
            value: r.value.max(0), 
        })
        .collect();
}