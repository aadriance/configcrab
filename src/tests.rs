#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
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
