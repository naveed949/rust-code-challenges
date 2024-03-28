
use polars::prelude::*;
use polars::datatypes::DataType;

pub fn fetch_data(path: &str, indicator: &str) -> Result<DataFrame> {
    let lf = LazyFrame::scan_csv(path, CsvReadOptions::new().has_header(true), None)?
        .filter(col(indicator).eq(lit(indicator)));
}

pub fn calculate_life_expectancy(data: &DataFrame, country: &str, year: u32) -> f64 {
    let country = data
        .filter(
            &data
                .column("Life expectancy")
                .unwrap()
                .equal(country)
                .unwrap(),
        )
        .unwrap();
    let year = country
        .filter(&country.column("Year").unwrap().equal(year).unwrap())
        .unwrap();
    let life_expectancy = year.column("Life satisfaction").unwrap().mean().unwrap();
    life_expectancy
}
