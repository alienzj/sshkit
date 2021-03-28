use clap::Clap;
use serde_derive::Deserialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "alienzj <alienchuj@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
    #[clap(short)]
    verbose: bool,
    #[clap(short)]
    otp: bool,
    #[clap(short)]
    retry: bool,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Config(Config),
    Login(Login),
    Tunel(Tunel),
    Upload(Upload),
    Download(Download),
}

#[derive(Clap, Deserialize, Debug)]
struct Config {
    #[clap(short, long, default_value = "~/.config/sshkit/sshkit.toml")]
    config: String,
    user: String,
    node: String,
    password: String,
    code: String,
    tunel: Vec<String>,
}

#[derive(Clap, Debug)]
struct Login {
    #[clap(short, long, default_value = "~/.config/sshkit/sshkit.toml")]
    config: String,
    user: String,
    node: String,
    password: String,
    code: String,
}

#[derive(Clap, Debug)]
struct Tunel {
    #[clap(short, long, default_value = "~/.config/sshkit/sshkit.toml")]
    config: String,
    user: String,
    node: String,
    password: String,
    code: String,
    tunel: Vec<String>,
}

#[derive(Clap, Debug)]
struct Upload {
    #[clap(short, long, default_value = "~/.config/sshkit/sshkit.toml")]
    config: String,
    user: String,
    node: String,
    password: String,
    code: String,
    from: String,
    to: String,
}

#[derive(Clap, Debug)]
struct Download {
    #[clap(short, long, default_value = "~/.config/sshkit/sshkit.toml")]
    config: String,
    user: String,
    node: String,
    password: String,
    code: String,
    from: String,
    to: String,
}

#[derive(Deserialize, Debug)]
struct Configs {
    configs: Vec<Config>,
}

fn main() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Config(config) => configer(config),
        SubCommand::Login(config) => loginer(config),
        SubCommand::Tunel(config) => tuneler(config),
        SubCommand::Upload(config) => uploader(config),
        SubCommand::Download(config) => downloader(config),
    }
}

fn configer(config: Config) {
    let config_file = config.config;
    let user_name = config.user;
    println!("{}", config_file);
    println!("{}", user_name);

    if Path::exists(Path::new(&config_file)) {
        let mut config_handle = match File::open(&config_file) {
            Ok(s) => s,
            Err(e) => panic!("Error occurred opening file: {} - Err: {}", config_file, e),
        };
        let mut config_str = String::new();
        match config_handle.read_to_string(&mut config_str) {
            Ok(s) => s,
            Err(e) => panic!("Error reading file: {}", e),
        };

        let configs: Configs = toml::from_str(config_str.as_str()).unwrap();

        println!("config: \n {:#?}", configs);
    }
}

fn loginer(config: Login) {}

fn tuneler(config: Tunel) {}

fn uploader(config: Upload) {}

fn downloader(config: Download) {}
