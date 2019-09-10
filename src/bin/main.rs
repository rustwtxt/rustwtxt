//
// rustwtxt - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use clap;

fn main() {
    let args = clap::App::new("rustwtxt")
        .version(clap::crate_version!())
        .author("Ben Morrison <ben@gbmor.dev>")
        .about("command-line twtxt client")
        .arg(
            clap::Arg::with_name("timeline")
                .short("t")
                .long("timeline")
                .value_name("TIMELINE")
                .help("Displays the followed users' tweets in a timeline")
                .takes_value(false),
        )
        .get_matches();

    eprintln!("{:#?}", args);
}
