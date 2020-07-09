use chrono::prelude::*;
use colored::*;
use rustwtxt::{TweetMap, Twtxt};

use std::collections::BTreeMap;
use std::fs;
use std::process;

use crate::cache;
use crate::conf;
use crate::ed;

#[derive(Debug, Clone)]
struct Timeline {
    tweets: TweetMap,
}

pub fn tweet() {
    let twtxt_path = &*conf::DATA.path.clone();
    let tweet_body = ed::call();

    let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, false);

    let tweet_body = format!("{}\t{}", timestamp, tweet_body);

    let current_tweets = match fs::read_to_string(twtxt_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Can't read twtxt.txt: {:?}", err);
            process::exit(1);
        }
    };

    let mut line_vec = current_tweets.split('\n').collect::<Vec<&str>>();
    let mut trimmed_line_vec = Vec::new();
    line_vec.iter().for_each(|line| {
        if line == &"" {
            return;
        }
        trimmed_line_vec.push(line.to_owned());
    });

    line_vec.push(&tweet_body);
    let new_tweets = trimmed_line_vec.join("\n");

    match fs::write(twtxt_path, new_tweets) {
        Err(err) => {
            eprintln!("Couldn't append new tweet to twtxt.txt: {:?}", err);
        }
        _ => {
            println!();
            println!("Tweet added!");
            println!();
        }
    }
}

pub fn show() {
    let twtxt_path = &*conf::DATA.path.clone();
    let twtxt_str = match fs::read_to_string(twtxt_path) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Couldn't read local twtxt.txt - omitting from timeline.");
            "".into()
        }
    };

    let nick = &*conf::DATA.nick;
    let url = &*conf::DATA.url;

    let tweet_lines = twtxt_str.split('\n').collect::<Vec<&str>>();
    let mut tweet_lines_sanitized = Vec::new();
    tweet_lines.iter().for_each(|line| {
        if line == &"" || line.starts_with('#') {
            return;
        }
        let timestamp = line.split('\t').collect::<Vec<&str>>();
        if DateTime::parse_from_rfc3339(timestamp[0]).is_err() {
            return;
        }

        let line = format!(
            "{}{}{}\n\t{}\n",
            nick.green(),
            "@".bold(),
            url.white(),
            line.white().bold()
        );
        let line = (timestamp[0].to_string(), line);
        tweet_lines_sanitized.push(line);
    });

    let mut follows = pull_followed_tweets();

    tweet_lines_sanitized.iter().for_each(|(k, v)| {
        follows.insert(k.to_owned(), v.to_owned());
    });

    follows.iter().for_each(|(_, v)| {
        println!("{}", v);
    });
}

fn pull_followed_tweets() -> BTreeMap<String, String> {
    let follows = &*conf::DATA.follow;
    let broken_follows = follows
        .iter()
        .map(|each| {
            let split = each.split(' ').collect::<Vec<&str>>();
            (split[0].into(), split[1].into())
        })
        .collect::<Vec<(String, String)>>();

    let mut tweetmap = BTreeMap::new();

    broken_follows.iter().for_each(|(nick, url)| {
        let _modtime = cache::get_remote_modtime(url);
        let twtxt = match Twtxt::from(url) {
            Some(data) => data,
            None => return,
        };
        let tweets = twtxt.tweets().clone();
        tweets.iter().for_each(|(k, v)| {
            tweetmap.insert(
                k.clone(),
                format!(
                    "{}{}{}\n\t{}\t{}\n",
                    nick.blue(),
                    "@".bold(),
                    url.white(),
                    k.clone().white().bold(),
                    (*v.body()).to_string().white().bold(),
                ),
            );
        });
    });

    tweetmap
}
