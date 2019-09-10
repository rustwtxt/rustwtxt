//
// rustwtxt - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//
#[macro_use]
extern crate lazy_static;

use clap;

mod conf;

fn main() {
    let args = clap::App::new("rustwtxt")
        .version(clap::crate_version!())
        .author("Ben Morrison <ben@gbmor.dev>")
        .about("command-line twtxt client")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .help("Alternate config file to pass to rustwtxt.")
                .takes_value(true),
        )
        .subcommand(
            clap::SubCommand::with_name("init").about("Initialization wizard for new installs."),
        )
        .subcommand(
            clap::SubCommand::with_name("timeline")
                .about("Displays the followed users' tweets in a timeline."),
        )
        .subcommand(
            clap::SubCommand::with_name("tweet")
                .about("Opens your preferred editor to compose a new tweet."),
        )
        .subcommand(clap::SubCommand::with_name("follow").about("Follow a given user."))
        .subcommand(clap::SubCommand::with_name("unfollow").about("Stop following a given user."))
        .get_matches();

    eprintln!("{:#?}", args);

    match args.subcommand() {
        ("init", _args) => conf::init(),
        _ => return,
    }
}
