use anyhow::Result;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        .arg(
            Arg::with_name("file") //TO-DO real args
                .short("f")
                .long("file")
                .takes_value(true)
                .help("File path")
                .required(true),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    fs::copy(file, "copy.txt").unwrap_or_else(|error| {
        println!("Failed to copy file: {:?}", error);
        0
    });

    //eg; macos windows etc.
    println!("You're running: {}", env::consts::OS);
    let config = Config {
        file: "Test".to_string(),
        winpath: "TestWin".to_string(),
        macpath: "TestMac".to_string(),
        linuxpath: "TestLinux".to_string(),
    };

    let example_config = [config.clone(), config];
    export_config(&example_config, "configcrab.yaml").unwrap();
    let import = import_config("configcrab.yaml").unwrap();
    println!("{:?}", import);
}

fn export_config(config: &[Config], file: &str) -> Result<()> {
    let config_yaml = serde_yaml::to_string(&config)?;
    fs::write(file, config_yaml)?;
    Ok(())
}

fn import_config(file: &str) -> Result<Vec<Config>> {
    let config_yaml = fs::read_to_string(file)?;
    let config: Vec<Config> = serde_yaml::from_str(&config_yaml)?;
    Ok(config)
}
