//
// rustweet - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//
use serde::{Deserialize, Serialize};
use serde_yaml;

use std::fs;
use std::process;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub nick: String,
    pub path: String,
    pub url: String,
    pub follow: Vec<String>,
}

lazy_static! {
    pub static ref DATA: Data = init();
    pub static ref FILE: String = {
        format!(
            "{}/.config/rustweet",
            std::env::var("HOME").unwrap_or_else(|_| ".".into())
        )
    };
}

pub fn init() -> Data {
    let file = (*FILE).to_string();

    if fs::metadata(&file).is_err() {
        eprintln!();
        eprintln!("Configuration file missing: $HOME/.config/rustweet");
        eprintln!("For instructions, please see:");
        eprintln!("\t$ rustweet --manual");
        eprintln!();
        process::exit(1);
    }

    let conf_as_str = match fs::read_to_string(&file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!();
            eprintln!(
                "Can't read configuration file: $HOME/.config/rustweet -- {:?}",
                err
            );
            eprintln!();
            process::exit(1);
        }
    };

    match serde_yaml::from_str::<Data>(&conf_as_str) {
        Ok(data) => data,
        Err(err) => {
            eprintln!();
            eprintln!(
                "Improperly formatted configuration file: $HOME/.config/rustweet: {:?}",
                err
            );
            eprintln!();
            process::exit(1);
        }
    }
}
