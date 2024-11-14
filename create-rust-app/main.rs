
    mod utils;
    mod error;

    fn main() -> Result<(), CustomError> {
        let matches = App::new("Hello World CLI")
            .version("1.0")
            .author("Your Name <your.email@example.com>")
            .about("Says hello")
            .arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .value_name("NAME")
                    .help("Sets the name to greet")
                    .takes_value(true),
            )
            .arg(Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true))
            .get_matches();

        let name = matches.value_of("name").unwrap_or("world");
        utils::greet(name)?;
        Ok(())
    }