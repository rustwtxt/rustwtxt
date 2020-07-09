use std::env;
use std::fs;
use std::process;

use chrono::prelude::*;

use crate::conf;

lazy_static! {
    static ref VAR: String = match env::var("EDITOR") {
        Ok(ed) => {
            if &ed == "" {
                "nano".into()
            } else {
                ed
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            "nano".into()
        }
    };
}

fn create_tmp_file<'a>() -> Result<String, &'a str> {
    let the_time = Utc::now().to_rfc3339();
    let conf = conf::DATA.clone();

    let file_name = format!("/tmp/rustweet_ed_{}_{}", conf.nick, the_time);
    match fs::write(&file_name, "") {
        Ok(_) => Ok(file_name),
        Err(_) => Err("Unable to create temp file"),
    }
}

pub fn call() -> String {
    let tmp_loc = match create_tmp_file() {
        Ok(filename) => filename,
        Err(err) => panic!("{:?}", err),
    };

    if let Err(err) = process::Command::new(VAR.clone())
        .arg(tmp_loc.clone())
        .stdin(process::Stdio::inherit())
        .stdout(process::Stdio::inherit())
        .output()
    {
        eprintln!("{:?}", err);
    };

    let body = match fs::read_to_string(tmp_loc.clone()) {
        Ok(string) => string.trim().to_owned(),
        Err(err) => panic!("{:?}", err),
    };

    if let Err(err) = fs::remove_file(tmp_loc) {
        eprintln!("{:?}", err);
    };

    body
}
