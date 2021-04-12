use clap::Clap;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "alienzj <alienchuj@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
    #[clap(long)]
    verbose: bool,
    #[clap(long)]
    otp: bool,
    #[clap(long)]
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

#[derive(Clap, Debug)]
struct Config {
    #[clap(long, default_value = "/home/alienzj/.config/sshkit.toml")]
    config: String,
    #[clap(long)]
    user: String,
    #[clap(long)]
    node: String,
    #[clap(long)]
    password: String,
    #[clap(long)]
    code: String,
}

#[derive(Clap, Debug)]
struct Login {
    #[clap(long, default_value = "/home/alienzj/.config/sshkit.toml")]
    config: String,
    #[clap(long)]
    user: String,
    #[clap(long)]
    node: String,
    #[clap(long)]
    password: String,
    #[clap(long)]
    code: String,
}

#[derive(Clap, Debug)]
struct Tunel {
    #[clap(long, default_value = "/home/alienzj/.config/sshkit.toml")]
    config: String,
    #[clap(long)]
    user: String,
    #[clap(long)]
    node: String,
    #[clap(long)]
    password: String,
    #[clap(long)]
    code: String,
    #[clap(long)]
    tunel: Vec<String>,
}

#[derive(Clap, Debug)]
struct Upload {
    #[clap(long, default_value = "/home/alienzj/.config/sshkit.toml")]
    config: String,
    #[clap(long)]
    user: String,
    #[clap(long)]
    node: String,
    #[clap(long)]
    password: String,
    #[clap(long)]
    code: String,
    #[clap(long)]
    from: String,
    #[clap(long)]
    to: String,
}

#[derive(Clap, Debug)]
struct Download {
    #[clap(long, default_value = "/home/alienzj/.config/sshkit.toml")]
    config: String,
    #[clap(long)]
    user: String,
    #[clap(long)]
    node: String,
    #[clap(long)]
    password: String,
    #[clap(long)]
    code: String,
    #[clap(long)]
    from: String,
    #[clap(long)]
    to: String,
}
#[derive(Deserialize, Debug)]
struct Configs {
    configs: Vec<Hosts>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Hosts {
    host: Vec<Host>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Host {
    user: String,
    node: String,
    password: String,
    code: String,
}

fn main() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Config(args) => configer(args),
        SubCommand::Login(args) => loginer(args),
        SubCommand::Tunel(args) => tuneler(args),
        SubCommand::Upload(args) => uploader(args),
        SubCommand::Download(args) => downloader(args),
    }
}

fn configer(args: Config) {
    let config_file = args.config;
    let user_name = args.user;
    println!("{} {}", config_file, user_name);

    if Path::exists(Path::new(&config_file)) {
        let mut handle = match File::open(&config_file) {
            Ok(s) => s,
            Err(e) => panic!("Error occurred opening file: {} - Err: {}", config_file, e),
        };

        let mut config_str = String::new();
        match handle.read_to_string(&mut config_str) {
            Ok(s) => s,
            Err(e) => panic!("Error reading file: {}", e),
        };
        println!("{:?}", config_str);

        let configs: Hosts = toml::from_str(config_str.as_str()).unwrap();
        println!("{:?}", configs);
        //let configs_toml = toml::to_string_pretty(&configs).unwrap();
        //println!("{:?}", configs_toml);
    }
}

fn loginer(args: Login) {}

fn tuneler(args: Tunel) {}

fn uploader(args: Upload) {}

fn downloader(args: Download) {}
