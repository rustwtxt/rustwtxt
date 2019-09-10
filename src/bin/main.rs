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
                .value_name("FILE")
                .help("Alternate config file to pass to rustwtxt.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("follow")
                .short("f")
                .long("follow")
                .value_name("URL")
                .help("URL of a user's twtxt.txt file you wish to follow."),
        )
        .arg(
            clap::Arg::with_name("unfollow")
                .short("u")
                .long("unfollow")
                .value_name("NICK")
                .help("Nick of the user you wish to stop following."),
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
        .get_matches();

    eprintln!("{:#?}", args);

    match args.subcommand() {
        ("init", _args) => {
            eprintln!("{:#?}", _args);
            conf::init();
        }
        _ => return,
    }
}
