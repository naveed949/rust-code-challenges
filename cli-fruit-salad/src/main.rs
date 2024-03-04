use cli_fruit_salad::create_fruit_salad;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        println!("Usage: cli-fruit-salad [OPTIONS] <number>");
        println!();
        println!("Options:");
        println!("  -h, --help     Show help information");
        println!();
        println!("Arguments:");
        println!("  <number>       The number of fruits to include in the fruit salad");
        return;
    }

    let number: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number provided");
            return;
        }
    };
    println!("Creating a fruit salad with {} fruits", number);
    let salad = create_fruit_salad(number);
    match salad {
        Some(fruit) => println!("Fruit salad: {}", fruit),
        None => println!("Invalid number provided"),
    }
}
