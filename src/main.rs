extern crate clap;

use clap::{App, Arg};

mod config;

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

    let config = config::get_config(&matches);
}
