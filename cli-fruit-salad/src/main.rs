use cli_fruit_salad::create_fruit_salad;
use clap::{App, Arg};
fn main() {
    let matches = App::new("cli-fruit-salad")
        .version("1.0")
        .author("Naveed")
        .about("Creates a fruit salad")
        .arg(
            Arg::with_name("number")
                .help("The number of fruits to include in the fruit salad")
                .required(true)
                .index(1),
        )
        .get_matches();

    let number: u32 = matches.value_of("number").unwrap().parse().unwrap();
    println!("Creating a fruit salad with {} fruits", number);
    let salad = create_fruit_salad(number);
    if let Some(fruit) = salad {
        println!("salad > {}", fruit);
    } else {
        println!("Invalid number provided");
    }
}
