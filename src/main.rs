use clap::{Arg, App};
use std::{fs, env};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    file: String,
    winpath: String,
    macpath: String,
    linuxpath: String,
}

fn main() {
    let matches = App::new("ConfigCrab")
        .version("0.1.0")
        .author("Andrew Adriance")
        .about("ConfigCrab helps keep config files in sync.")
        .arg(Arg::with_name("file") //TO-DO real args
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("File path")
                 .required(true))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    fs::copy(file, "copy.txt").unwrap_or_else(|error| {
        println!("Failed to copy file: {:?}", error);
        0
    });

    //eg; macos windows etc.
    println!("You're running: {}", env::consts::OS);
    let config = Config{ file: "Test".to_string(),
                         winpath: "TestWin".to_string(),
                         macpath: "TestMac".to_string(),
                         linuxpath: "TestLinux".to_string()};

    let yam = serde_yaml::to_string(&config).unwrap_or_else(|error| {
        println!("No Yaml for you :( {}", error);
        "err".to_string()
    });

    println!("Enjoy a yam: \n{}", yam);
}
