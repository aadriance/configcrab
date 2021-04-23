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

impl Config {
    fn new() -> Config {
        Config {
            file: "".to_string(),
            winpath: "".to_string(),
            macpath: "".to_string(),
            linuxpath: "".to_string()
        }
    }

    fn with_file(mut self, file: &str) -> Self {
        self.file = file.to_string();
        self
    }

    fn with_winpath(mut self, winpath: &str) -> Self {
        self.winpath = winpath.to_string();
        self
    }

    fn with_macpath(mut self, macpath: &str) -> Self {
        self.macpath = macpath.to_string();
        self
    }

    fn with_linuxpath(mut self, linuxpath: &str) -> Self {
        self.linuxpath = linuxpath.to_string();
        self
    }
}

#[derive(Debug)]
struct CrabOrders {
    config_path: String,
    platform: String
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
    let orders = CrabOrders {
        config_path: "configcrab.yaml".to_string(),
        platform: env::consts::OS.to_string()
    };

    println!("Your orders: {:?}", orders);
    let config = Config::new()
        .with_file("file")
        .with_linuxpath("linux")
        .with_macpath("mac")
        .with_winpath("win");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_init() {
        let test_config = Config::new();
        assert_eq!("", test_config.file);
        let full_config = Config::new()
            .with_file("file")
            .with_macpath("macpath")
            .with_winpath("winpath")
            .with_linuxpath("linuxpath");

        assert_eq!("file", full_config.file);
        assert_eq!("macpath", full_config.macpath);
        assert_eq!("winpath", full_config.winpath);
        assert_eq!("linuxpath", full_config.linuxpath);
    }
}
