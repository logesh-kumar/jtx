mod filters;
mod utils;
use clap::{App, Arg, crate_version, crate_authors};
use std::io::{self, Read};

fn main() {
    let matches = App::new("jtx")
        .version(crate_version!())  // Automatically use the version from Cargo.toml
        .author(crate_authors!())   // Automatically use the authors from Cargo.toml
        .about("Processes JSON data")
        .arg(Arg::with_name("filter")
             .short('f')
             .long("filter")
             .takes_value(true)
             .help("Apply a filter to the JSON input"))      
            .after_help("EXAMPLES:
    echo '{\"name\": \"Alice\", \"age\": 30}' | jtx -f '$.name'
    echo '{\"name\": \"Alice\", \"age\": 30}' | jtx -f '$.age'
    echo '{\"name\": \"Alice\", \"age\": 30}' | jtx -f '$.name, $.age'
    echo '{\"name\": \"Alice\", \"age\": 30}' | jtx -f '$.name, $.age, $.address'
    echo '{\"name\": \"Alice\", \"age\": 30}' | jtx -f '$.name, $.age, $.address.city'
    ")
             
        .get_matches();

    // Read the input from stdin
    let mut buffer = String::new();
    let input_length = io::stdin().read_to_string(&mut buffer).unwrap_or(0);

    // Check if input is empty and no filter is provided, display usage help
    if input_length == 0 && !matches.is_present("filter") {
        println!("No input provided. Here are some examples on how to use jtx:");
        println!("  echo '{{\"name\": \"Alice\", \"age\": 30}}' | jtx -f '$.name'");        
        return;
    }

    // check if  filter is not provided
    if input_length == 0 && matches.is_present("filter") {
        println!("No input provided. Please provide input to apply the filter.");
        return;
    }
    

    // Process the input with the specified filter
    if let Some(filter) = matches.value_of("filter") {
        utils::process_input(&buffer, filter);
    } else {
        utils::process_input(&buffer, ".");
    }
}
