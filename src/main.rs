use anyhow::Result;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{env, fs};
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
struct PlatformPath {
    platform: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    file: String,
    paths: Vec<PlatformPath>,
}

impl Config {
    fn new() -> Config{
        Config{
            file: "file".to_string(),
            paths: Vec::new(),
        } 
    }

    fn with_file(mut self, file: &str) -> Self {
        self.file = file.to_string();
        self
    }

    fn with_platform(mut self, platform: &str, path: &str) -> Self {
        let plat_path = PlatformPath {
            platform: platform.to_string(),
            path: path.to_string(),
        };

        self.paths.push(plat_path);
        self.paths.sort();
        self
    }

    fn with_winpath(self, winpath: &str) -> Self {
        self.with_platform("windows", winpath)
    }

    fn with_macpath(self, macpath: &str) -> Self {
        self.with_platform("macos", macpath)
    }

    fn with_linuxpath(self, linuxpath: &str) -> Self {
        self.with_platform("linux", linuxpath)
    }
}

#[derive(Debug)]
struct CrabOrders {
    config_path: String,
    platform: String,
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
        platform: env::consts::OS.to_string(),
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

/*
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
*/