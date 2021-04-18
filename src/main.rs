use clap::Clap;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
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

#[derive(Deserialize, Serialize, Debug)]
struct Hosts {
    host: Vec<Host>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Host {
    pub user: String,
    pub node: String,
    pub password: String,
    pub code: String,
}

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Config(args) => configer(&args),
        SubCommand::Login(args) => loginer(&args),
        SubCommand::Tunel(args) => tuneler(&args),
        SubCommand::Upload(args) => uploader(&args),
        SubCommand::Download(args) => downloader(&args),
    };
    Ok(())
}

fn configer(args: &Config) -> std::io::Result<()> {
    println!("input args:\n{:?}\n", args);

    if Path::exists(Path::new(&args.config)) {
        let mut handle = match File::open(&args.config) {
            Ok(s) => s,
            Err(e) => panic!("Error occurred opening file: {} - Err: {}", args.config, e),
        };

        let mut config_str = String::new();
        match handle.read_to_string(&mut config_str) {
            Ok(s) => s,
            Err(e) => panic!("Error reading file: {}", e),
        };
        println!("input host:\n{:?}\n", config_str);


        let mut host_vec: Hosts = toml::from_str(config_str.as_str()).unwrap();
        for host in &mut host_vec.host {
            if (host.user == args.user) && (host.node == args.node) {
                host.code = args.code.clone();
                host.password = args.password.clone();
            }
        }
        println!("decode host:\n{:?}\n", host_vec);

        let output_str = toml::to_string(&host_vec).unwrap();
        std::fs::write(&args.config, output_str.as_bytes())?;
 
        Ok(())
    }
    else {
        let mut handle = match File::create(&args.config) {
            Ok(s) => s,
            Err(e) => panic!("Error occurred creating file: {} - Err: {}", args.config, e),
        };
        let mut hosts: Hosts = Hosts {
            host: Vec::new(),
        }; 

        let host: Host = Host {
            user: args.user.clone(),
            node: args.node.clone(),
            password: args.password.clone(),
            code: args.code.clone()
        };
        hosts.host.push(host);

        let config_str = toml::to_string(&hosts).unwrap();
        println!("output config host:\n {:?}\n", hosts);

        handle.write_all(config_str.as_bytes())?;
        Ok(())
    }
}

fn loginer(args: &Login) -> std::io::Result<()> { Ok(()) }

fn tuneler(args: &Tunel) -> std::io::Result<()> { Ok(()) }

fn uploader(args: &Upload) -> std::io::Result<()> { Ok(()) }

fn downloader(args: &Download) -> std::io::Result<()> { Ok(()) }
