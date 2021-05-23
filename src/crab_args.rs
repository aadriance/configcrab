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
    app.subcommand(
        SubCommand::with_name("install").about("Copies files from config to the local machine"),
    )
}

fn grab_sub_cmd(app: App<'static, 'static>) -> App {
    app.subcommand(
        SubCommand::with_name("grab")
            .about("Copies file from the local machine to the config")
            .arg(
                Arg::with_name("target")
                    .long("target")
                    .short("t")
                    .takes_value(true)
                    .required(true),
            ),
    )
}

fn options(app: App<'static, 'static>) -> App {
    app.arg(
        Arg::with_name("config")
            .long("config")
            .short("c")
            .default_value("configcrab.yaml")
            .help("Specify a config file for your crab"),
    )
    .arg(
        Arg::with_name("verbose")
            .short("v")
            .help("Enables debug output"),
    )
}

pub fn configcrab_app() -> App<'static, 'static> {
    let mut configcrab = base_app();
    configcrab = plat_args(configcrab);
    configcrab = install_sub_cmd(configcrab);
    configcrab = options(configcrab);
    configcrab = grab_sub_cmd(configcrab);
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

pub fn is_verbose(matches: &ArgMatches) -> bool {
    matches.is_present("verbose")
}
