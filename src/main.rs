extern crate byteorder;
extern crate clap;

use clap::{App, Arg};
use std::fs::File;

mod config;
mod printer;

fn main() {
    let matches = App::new("nummer")
        .version("0.1")
        .author("Joel Depooter")
        .about("Extracts numeric values from a binary file")
        .arg(
            Arg::with_name("binary file")
                .short("f")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("offset")
                .short("o")
                .long("offset")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("stride")
                .short("s")
                .long("stride")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("data type")
                .short("t")
                .long("data-type")
                .required(true)
                .takes_value(true),
        ).get_matches();

    let config = match config::get_config(&matches) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let input_file_path = matches.value_of("binary file").unwrap();
    let file = File::open(input_file_path).expect("The file could not be found.");

    match printer::print_values(file, config) {
        Err(e) => {
            eprintln!("{}", e);
        }
        _ => {}
    }
}
