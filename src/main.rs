use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs};
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
struct PlatformPath {
    platform: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Config {
    file: String,
    paths: Vec<PlatformPath>,
}

impl Config {
    fn new() -> Config {
        Config {
            file: "".to_string(),
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
    let matches = crab_args::configcrab_app().get_matches();

    if matches.is_present("platform_flags") {
        //to-do improve magic arg strings
        if matches.is_present("win") {
            println!("Windows selected!");
        }

        if matches.is_present("mac") {
            println!("Mac selected!");
        }

        if matches.is_present("linux") {
            println!("Linux selected!");
        }
    }

    let file = "test";//matches.value_of("file").unwrap();
    fs::copy(file, "copy.txt").unwrap_or_else(|error| {
        println!("Failed to copy file: {:?}", error);
        0
    });

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

mod crab_args {
    use clap::*;

    fn base_app() -> App<'static, 'static> {
        App::new("ConfigCrab")
            .version("0.1.0")
            .author("Andrew Adriance")
            .about("ConfigCrab helps keep config files in sync.") 
    }

    fn plat_args(app: App<'static, 'static>) -> App {
            app
            .arg(
                Arg::with_name("win")
                    .long("win")
                    .help("Sets the platform to Windows"))
            .arg(Arg::with_name("mac")
                    .long("mac")
                    .help("Sets ths platform to Mac OS"))
            .arg(Arg::with_name("linux")
                    .long("linux")
                    .help("Sets the platform to Linux"))
            .group(ArgGroup::with_name("platform_flags")
                .args(&["win", "mac", "linux"]))
    }

    pub fn configcrab_app() -> App<'static, 'static> {
        let mut configcrab = base_app();
        configcrab = plat_args(configcrab);
        configcrab
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_init_mac() {
        let test_config = Config::new();
        assert_eq!("", test_config.file);
        let full_config = Config::new().with_file("file").with_macpath("macpath");

        assert_eq!("file", full_config.file);
        assert_eq!("macos", full_config.paths[0].platform);
        assert_eq!("macpath", full_config.paths[0].path);
    }

    #[test]
    fn test_config_init_win() {
        let test_config = Config::new();
        assert_eq!("", test_config.file);
        let full_config = Config::new().with_file("file").with_winpath("winpath");

        assert_eq!("file", full_config.file);
        assert_eq!("windows", full_config.paths[0].platform);
        assert_eq!("winpath", full_config.paths[0].path);
    }

    #[test]
    fn test_config_init_linux() {
        let test_config = Config::new();
        assert_eq!("", test_config.file);
        let full_config = Config::new().with_file("file").with_linuxpath("linuxpath");

        assert_eq!("file", full_config.file);
        assert_eq!("linux", full_config.paths[0].platform);
        assert_eq!("linuxpath", full_config.paths[0].path);
    }

    #[test]
    fn test_config_init_platform() {
        let test_config = Config::new();
        assert_eq!("", test_config.file);
        let full_config = Config::new()
            .with_file("file")
            .with_platform(env::consts::OS, "platpath");

        assert_eq!("file", full_config.file);
        assert_eq!(env::consts::OS, full_config.paths[0].platform);
        assert_eq!("platpath", full_config.paths[0].path);
    }

    #[test]
    fn test_import_export() {
        let config = Config::new()
            .with_file("file")
            .with_linuxpath("linux")
            .with_macpath("mac")
            .with_winpath("win");

        let example_config = [config.clone(), config];
        export_config(&example_config, "test_configcrab.yaml").unwrap();
        let import = import_config("test_configcrab.yaml").unwrap();
        fs::remove_file("test_configcrab.yaml").unwrap();
        assert_eq!(example_config.to_vec(), import);
    }
}
