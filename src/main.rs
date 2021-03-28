#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use clap::ArgMatches;

use sshkit;

use serde_derive::{Deserialize, Serialize};

use users::{Groups, Users, UsersCache};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    user: String,
    node: String,
    password: String,
    code: String,
    tunel: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Configs {
    host_vec: Option<Vec<Config>>,
}

fn main() {
    let app_m = sshkit::cli::build_cli().get_matches();

    match app_m.subcommand_name() {
        Some("config") => configer(&app_m),
        Some("login") => loginer(&app_m),
        Some("tunel") => tuneler(&app_m),
        Some("upload") => uploader(&app_m),
        Some("download") => downloader(&app_m),
        _ => {}
    }
}

fn configer(matches: &ArgMatches) {
    let mut cache = UsersCache::new();
    let uid = cache.get_current_gid();
    let user = cache.get_user_by_uid(uid).unwrap();

    let config_file = matches.value_of("config").unwrap();
    let user_name = matches.value_of("user").unwrap();
    let node_address = matches.value_of("node").unwrap();
    let password = matches.value_of("password").unwrap();
    let code = matches.value_of("code").unwrap();
    let tunel = matches.value_of("tunel").unwrap();

    println!("{}", config_file);
    println!("{}", user_name);
    println!("{}", node_address);

    let conf = Config {
        user: user_name.to_string(),
        node: node_address.to_string(),
        password: password.to_string(),
        code: code.to_string(),
        tunel: tunel.to_string(),
    };

    if Path::exists(Path::new(config_file)) {
        let mut config_handle = match File::open(config_file) {
            Ok(s) => s,
            Err(e) => panic!("Error occurred opening file: {} - Err: {}", config_file, e),
        };
        let mut config_str = String::new();
        match config_handle.read_to_string(&mut config_str) {
            Ok(s) => s,
            Err(e) => panic!("Error reading file: {}", e),
        };

        let config: Configs = toml::from_str(config_str.as_str()).unwrap();

        println!("config: \n {:#?}", config);
    }
}

fn loginer(matches: &ArgMatches) {}

fn tuneler(matches: &ArgMatches) {
    let tunels: Vec<_> = matches.values_of("tunel").unwrap().collect();
}

fn uploader(matches: &ArgMatches) {}

fn downloader(matches: &ArgMatches) {}
