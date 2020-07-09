use std::fs;
use std::process;

use crate::conf;

pub fn follow(url: &str) {
    let data = &*conf::DATA.follow;
    let mut data = data.to_vec();
    let twtxt = if let Ok(val) = rustwtxt::pull_twtxt(url) {
        val
    } else {
        eprintln!("Can't pull twtxt file.");
        eprintln!("I won't be able to parse the nick out of the metadata.");
        String::new()
    };

    let nick = if let Ok(val) = rustwtxt::parse::metadata(&twtxt, "nick") {
        val
    } else {
        eprintln!("Can't parse nick out of metadata.");
        eprintln!("Please add it to the entry manually.");
        String::new()
    };

    let entry = format!("{} {}", nick, url);
    data.push(entry);

    let nick = (&*conf::DATA.nick).to_owned();
    let path = (&*conf::DATA.path).to_owned();
    let url = (&*conf::DATA.url).to_owned();

    let new_conf = conf::Data {
        nick,
        path,
        url,
        follow: data,
    };

    let yaml_str = if let Ok(yaml) = serde_yaml::to_string(&new_conf) {
        yaml
    } else {
        eprintln!("Couldn't parse data as yaml.");
        process::exit(1);
    };

    if let Err(err) = fs::write(&*conf::FILE, yaml_str) {
        eprintln!("Couldn't rewrite config file: {:?}", err);
    }
}

pub fn unfollow(nick: &str) {
    let data = &*conf::DATA.follow;
    let data = data.to_vec();
    let mut new_data = vec![];

    data.iter().for_each(|entry| {
        if entry.contains(nick) {
            return;
        }
        new_data.push(entry.to_owned());
    });

    let nick = (&*conf::DATA.nick).to_owned();
    let path = (&*conf::DATA.path).to_owned();
    let url = (&*conf::DATA.url).to_owned();

    let new_conf = conf::Data {
        nick,
        path,
        url,
        follow: new_data,
    };

    let yaml_str = if let Ok(yaml) = serde_yaml::to_string(&new_conf) {
        yaml
    } else {
        eprintln!("Couldn't reparse data as yaml.");
        process::exit(1);
    };

    if let Err(err) = fs::write(&*conf::FILE, yaml_str) {
        eprintln!("Couldn't rewrite config file: {:?}", err);
    }
}
