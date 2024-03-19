use clap::{App, Arg};
use sysinfo::System;
use std::time::Duration;
use std::thread::sleep;
use rasciigraph::{plot, Config};

fn main() {
    let matches = App::new("cli-system-monitor")
        .version("1.0")
        .author("Naveed Iqbal <naveediqbal949@gmail.com>")
        .about("Monitors system resources and displays them as ASCII graphs")
        .arg(Arg::with_name("refresh-rate")
            .short("r")
            .long("refresh-rate")
            .value_name("SECONDS")
            .help("Sets the refresh rate in seconds")
            .takes_value(true))
        .get_matches();

    let refresh_rate = matches.value_of("refresh-rate").unwrap_or("1").parse::<u64>().unwrap();

    let mut system = System::new_all();
    let mut total_memories = vec![];
    let mut used_memories = vec![];
    let mut cpu_usages = vec![];
    let max_size = 100;
    loop {
        system.refresh_all();
        let cpu_usage = system.global_cpu_info().cpu_usage();
        let total_memory = system.total_memory() / 1024;
        let used_memory = system.used_memory() / 1024;
        // TODO: disk, network, and process monitoring
        // Remove the first element if the vectors are too large
        if cpu_usages.len() >= max_size {
            cpu_usages.remove(0);
            used_memories.remove(0);
            total_memories.remove(0);
        }
        // Push the data into the vectors
        total_memories.push(total_memory as f64);
        used_memories.push(used_memory as f64);
        cpu_usages.push(cpu_usage as f64);

        // Display the data as ASCII graphs using rasciigraph
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen
        println!("{}",
            plot(cpu_usages.clone(), Config::default().with_height(10).with_caption("CPU Usage (%)".to_string()))
        );
        println!("{}",
            plot(total_memories.clone(), Config::default().with_height(10).with_caption("Total Memory (KB)".to_string()))
        );
        println!("{}",
            plot(used_memories.clone(), Config::default().with_height(10).with_caption("Used Memory (KB)".to_string()))
        );

        sleep(Duration::from_secs(refresh_rate));
    }
}