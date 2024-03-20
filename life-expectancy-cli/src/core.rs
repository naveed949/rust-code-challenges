use polars::prelude::*;

pub fn fetch_data(path: &str) -> DataFrame {
    CsvReader::from_path(path)
        .unwrap()
        .finish()
        .unwrap()
    
}

pub fn calculate_life_expectancy(data: &DataFrame, country: &str, year: u32) -> f64 {
    let country = data.filter(&data.column("Life expectancy").unwrap().equal(country).unwrap()).unwrap();
    let year = country.filter(&country.column("Year").unwrap().equal(year).unwrap()).unwrap();
    let life_expectancy = year.column("Life satisfaction").unwrap().mean().unwrap();
    life_expectancy
}