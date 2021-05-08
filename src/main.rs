use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
struct PlatformPath {
    platform: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Config {
    file: String,
    file_dir: String,
    paths: Vec<PlatformPath>,
}

impl Config {
    fn new() -> Config {
        Config {
            file: "".to_string(),
            file_dir: "".to_string(),
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

    let platform = crab_args::get_platform(&matches);
    let config_path = crab_args::get_config_path(&matches);
    let orders = CrabOrders {
        config_path,
        platform,
    };

    println!("Your orders: {:?}", orders);
    let config = Config::new()
        .with_file("file.txt")
        .with_linuxpath("linux")
        .with_macpath("mac")
        .with_winpath("win");

    let example_config = [config.clone(), config];
    export_config(&example_config, "configcrab.yaml").unwrap();
    let import = import_config("configcrab.yaml").unwrap();
    println!("{:?}", import);

    if matches.is_present("install") {
        install(&example_config, &orders.platform);
    }
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

fn install(config: &[Config], platform: &str) {
    for item in config {
        for path in &item.paths {
            if path.platform == platform {
                let mut from_path = PathBuf::from(item.file_dir.clone());
                from_path.push(item.file.clone());
                let mut to_path = PathBuf::from(path.path.clone());
                to_path.push(item.file.clone());
                fs::copy(from_path, to_path).unwrap_or_else(|error| {
                    println!("Failed to copy file: {:?}", error);
                    0
                });
                break;
            }
        }
    }
}

mod crab_args {
    use clap::*;
    use std::env;

    fn base_app() -> App<'static, 'static> {
        App::new("ConfigCrab")
            .version("0.1.0")
            .author("Andrew Adriance")
            .about("ConfigCrab helps keep config files in sync.")
    }

    fn plat_args(app: App<'static, 'static>) -> App {
        app.arg(
            Arg::with_name("win")
                .long("win")
                .help("Sets the platform to Windows"),
        )
        .arg(
            Arg::with_name("mac")
                .long("mac")
                .help("Sets ths platform to Mac OS"),
        )
        .arg(
            Arg::with_name("linux")
                .long("linux")
                .help("Sets the platform to Linux"),
        )
        .arg(
            Arg::with_name("platform")
                .long("platform")
                .takes_value(true)
                .help("Specifies a custom platform"),
        )
        .group(ArgGroup::with_name("platform_flags").args(&["win", "mac", "linux", "platform"]))
    }

    fn install_sub_cmd(app: App<'static, 'static>) -> App {
        app.subcommand(SubCommand::with_name("install"))
    }

    fn options(app: App<'static, 'static>) -> App {
        app.arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .default_value("configcrab.yaml")
                .help("Specify a config file for your crab"),
        )
    }

    pub fn configcrab_app() -> App<'static, 'static> {
        let mut configcrab = base_app();
        configcrab = plat_args(configcrab);
        configcrab = install_sub_cmd(configcrab);
        configcrab = options(configcrab);
        configcrab
    }

    pub fn get_platform(matches: &ArgMatches) -> String {
        if matches.is_present("win") {
            "windows".to_string()
        } else if matches.is_present("mac") {
            "macos".to_string()
        } else if matches.is_present("linux") {
            "linux".to_string()
        } else if let Some(p) = matches.value_of("platform") {
            p.to_string()
        } else {
            env::consts::OS.to_string()
        }
    }

    pub fn get_config_path(matches: &ArgMatches) -> String {
        matches.value_of("config").unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

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
