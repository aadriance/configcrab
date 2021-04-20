use clap::{Arg, App};
use std::fs;

fn main() {
    let matches = App::new("ConfigCrab")
        .version("0.1.0")
        .author("Andrew Adriance")
        .about("ConfigCrab helps keep config files in sync.")
        .arg(Arg::with_name("file") //TO-TO real args
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("File path")
                 .required(true))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    fs::copy(file, "copy.txt");
}
