use clap::{App, Arg, SubCommand};

pub fn build_cli() {
    let matches = App::new("sshkit")
        .version("0.1.0")
        .author("Jie Zhu <alienchuj@gmail.com>")
        .about("ssh toolkit")
        .subcommand(
            SubCommand::with_name("config")
                .about("ssh config")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("config file")
                        .help("local config file, default: ~/.config/sshkit/sshkit.toml")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("user")
                        .short("u")
                        .long("user")
                        .value_name("user name")
                        .help("remote user name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("node")
                        .short("n")
                        .long("node")
                        .value_name("node address")
                        .help("remote node address")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .value_name("ssh password")
                        .help("ssh password")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("code")
                        .long("code")
                        .value_name("ssh otp code")
                        .help("ssh otp code")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("tunel")
                        .long("tunel")
                        .value_name("ssh tunel")
                        .help("ssh tunel")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("login")
                .about("ssh login")
                .arg(
                    Arg::with_name("user")
                        .short("u")
                        .long("user")
                        .value_name("user name")
                        .help("remote user name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("node")
                        .short("n")
                        .long("node")
                        .value_name("node address")
                        .help("remote node address")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .value_name("ssh password")
                        .help("ssh password")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("code")
                        .long("code")
                        .value_name("ssh otp code")
                        .help("ssh otp code")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("tunel")
                .about("ssh tunel")
                .arg(
                    Arg::with_name("user")
                        .short("u")
                        .long("user")
                        .value_name("user name")
                        .help("remote user name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("node")
                        .short("n")
                        .long("node")
                        .value_name("node address")
                        .help("remote node address")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .value_name("ssh password")
                        .help("ssh password")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("code")
                        .long("code")
                        .value_name("ssh otp code")
                        .help("ssh otp code")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("download")
                .about("download using scp")
                .arg(
                    Arg::with_name("user")
                        .short("u")
                        .long("user")
                        .value_name("user name")
                        .help("remote user name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("node")
                        .short("n")
                        .long("node")
                        .value_name("node address")
                        .help("remote node address")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .value_name("ssh password")
                        .help("ssh password")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("code")
                        .long("code")
                        .value_name("ssh otp code")
                        .help("ssh otp code")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("upload")
                .about("upload using scp")
                .arg(
                    Arg::with_name("user")
                        .short("u")
                        .long("user")
                        .value_name("user name")
                        .help("remote user name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("node")
                        .short("n")
                        .long("node")
                        .value_name("node address")
                        .help("remote node address")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .value_name("ssh password")
                        .help("ssh password")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("code")
                        .long("code")
                        .value_name("ssh otp code")
                        .help("ssh otp code")
                        .takes_value(true),
                ),
        )
        .get_matches();
}
