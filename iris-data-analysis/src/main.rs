use polars::prelude::*;
use polars::frame::DataFrame;
use std::path::Path;

fn main() {
    let iris_file_path: &Path = Path::new("dataset/Iris.csv");
    let iris_df: DataFrame = read_data_frame_from_csv(iris_file_path);
    println!("{:?}", iris_df.shape());
    println!("{:?}", iris_df.head(Some(5)));
    println!("{:?}", iris_df.tail(Some(5)));
    println!("{:?}", iris_df.describe(None));
    
}

fn read_data_frame_from_csv(
    csv_file_path: &Path,
) -> DataFrame {
    CsvReader::from_path(csv_file_path)
        .expect("Cannot open file.")
        .has_header(true)
        .finish()
        .unwrap()
}