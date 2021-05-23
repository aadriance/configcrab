use anyhow::*;
use dirs::home_dir;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process;

mod crab_args;
mod tests;

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

//
// Macros take a bit of mind bending to get
// use to after years of C #defines.
// Took me a few reads to understand it,
// here is a break down of how a variadic works
// for when I look at this in a month and totally
// forget.
//
// Repition start with $(
// $(
// The thing you are repeating
//$element:expr
//)
// Comma seperated
//,
// contains 0 or more instances
//*

macro_rules! verbose {
    ($toggle:expr, $format:expr, $($element:expr),*) => {
        if $toggle {
            println!($format, $($element),*);
        }
    }
}

//
// ew a global! I don't know, maybe I've written too many lines of C
// but storing things like verbosity in a global location like this
// makes a lot of sense to me. If this was Erlang where shared memory
// doesn't exist, then yeah pass it around.
//

static VERBOSITY: OnceCell<bool> = OnceCell::new();

fn verbosity() -> bool {
    match VERBOSITY.get() {
        Some(v) => *v,
        None => false,
    }
}

fn main() {
    let matches = crab_args::configcrab_app().get_matches();
    let v = crab_args::is_verbose(&matches);
    VERBOSITY.set(v).unwrap();

    let platform = crab_args::get_platform(&matches);
    let config_path = crab_args::get_config_path(&matches);
    let orders = CrabOrders {
        config_path,
        platform,
    };

    verbose!(verbosity(), "Your orders: {:#?}", orders);

    let config = import_config(&orders.config_path).unwrap_or_else(|error| {
        println!(
            "Failed to import config({:?}) from {}",
            error, orders.config_path
        );
        process::exit(1);
    });

    verbose!(
        verbosity(),
        "Imported: {:#?} from {}",
        config,
        orders.config_path
    );

    //below here is just built in test code for now
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

    let res = grab("../configcrab/README.md", "test");
    match res {
        Ok(v) => {
            println!("Config: {:?}", v);
            println!("Sanitized: {}", sanitize_dir(&v.paths[0].path));
            println!("Orig: {}", desanitize_dir(&sanitize_dir(&v.paths[0].path)));
        }
        Err(e) => println!("error parsing path: {:?}", e),
    };

    let home = home_dir();
    println!("Home: {:?}", home);
}

fn sanitize_dir(dir: &str) -> String {
    match home_dir() {
        Some(home) => dir.replace(home.to_str().unwrap(), "$_HOME_$"),
        None => dir.to_string(),
    }
}

fn desanitize_dir(dir: &str) -> String {
    match home_dir() {
        Some(home) => dir.replace("$_HOME_$", home.to_str().unwrap()),
        None => dir.to_string(),
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

fn grab(target: &str, platform: &str) -> Result<Config> {
    let path = PathBuf::from(target).canonicalize()?;
    let file = path
        .file_name()
        .ok_or_else(|| anyhow!("No file in path"))?
        .to_str()
        .ok_or_else(|| anyhow!("Not a valid UTF-8 STR"))?;

    let dir_path = path
        .parent()
        .ok_or_else(|| anyhow!("No directory parent"))?
        .to_str()
        .ok_or_else(|| anyhow!("Not a valid UTF-8 STR"))?;

    Ok(Config::new()
        .with_platform(platform, dir_path)
        .with_file(file))

    // Should I copy here? Is that someone elses job??
}
