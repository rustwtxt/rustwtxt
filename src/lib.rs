//! # rustwtxt
//!
//! This is a library intended to make working with `twtxt` timelines
//! a bit easier.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::str::FromStr;

use failure::format_err;
use regex::Regex;
use ureq;

pub mod parse;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type TweetMap = std::collections::BTreeMap<String, Tweet>;

/// Holds tweets and metadata from a single `twtxt.txt` file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Twtxt {
    nickname: String,
    url: String,
    tweets: TweetMap,
}

impl Twtxt {
    /// Returns the nickname associated with the `twtxt.txt` file.
    pub fn nick(&self) -> &str {
        &self.nickname
    }

    /// Returns the URL associated with the `twtxt.txt` file.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns a specific tweet by the timestamp key.
    pub fn tweet(&self, datestamp: &str) -> Option<&Tweet> {
        if self.tweets.contains_key(datestamp) {
            Some(&self.tweets[datestamp])
        } else {
            None
        }
    }

    /// Returns all tweets as a `TweetMap`, a thin wrapper around a `BTreeMap`.
    /// The tweets will be date-sorted.
    pub fn tweets(&self) -> &TweetMap {
        &self.tweets
    }

    /// Parse a remote `twtxt.txt` file into a `Twtxt` structure.
    pub fn from(url: &str) -> Option<Twtxt> {
        let twtxt = if let Ok(val) = pull_twtxt(&url) {
            val
        } else {
            return None;
        };

        let url = url.to_owned();

        let nickname = if let Ok(val) = parse::metadata(&twtxt, "nick") {
            val
        } else {
            return None;
        };

        let mut tweets = BTreeMap::new();
        twtxt
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|line| {
                if line.starts_with('#') || line == &"" || !line.contains('\t') {
                    return;
                }
                let tweet = if let Ok(val) = Tweet::from_str(line) {
                    val
                } else {
                    return;
                };
                tweets.insert(tweet.timestamp.clone(), tweet);
            });

        Some(Twtxt {
            nickname,
            url,
            tweets,
        })
    }
}

/// Holds a single tweet.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tweet {
    timestamp: String,
    body: String,
    mentions: Vec<String>,
    tags: Vec<String>,
}

impl Tweet {
    /// Returns the timestamp for a given tweet.
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    /// Returns the body of the tweet.
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Any mentions within the body of the tweet have been parsed out
    /// and are retrievable through this method.
    pub fn mentions(&self) -> Vec<String> {
        self.mentions.clone()
    }

    /// Any tags within the body of the tweet have been parsed out
    /// and are retrievable through this method.
    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl std::str::FromStr for Tweet {
    type Err = Box<dyn std::error::Error>;

    /// Takes a properly-formatted `twtxt` tweet and parses it
    /// into a `Tweet` structure.
    fn from_str(tweet: &str) -> Result<Tweet> {
        let split = tweet.split('\t').collect::<Vec<&str>>();
        let timestamp = split[0].to_string();
        let body = split[1].to_string();

        let mentions_regex = Regex::new(r"[@<].*[>]+")?;
        let tags_regex = Regex::new(r"(^|\s)#[^\s]+")?;

        let mentions = mentions_regex
            .find_iter(&body)
            .map(|ding| ding.as_str().to_string())
            .collect::<Vec<String>>();

        let tags = tags_regex
            .find_iter(&body)
            .map(|ding| {
                let tmp = ding.as_str();
                let tmp = tmp.split(' ').collect::<Vec<&str>>();
                if tmp[0] == "" && tmp.len() > 1 {
                    return tmp[1].to_string();
                }
                tmp[0].to_string()
            })
            .collect::<Vec<String>>();

        Ok(Tweet {
            timestamp,
            body,
            mentions,
            tags,
        })
    }
}

/// Pulls the target twtxt.txt file from the specified URL.
///
/// # Examples
/// ```
/// # use rustwtxt;
/// let out = if let Ok(data) = rustwtxt::pull_twtxt("https://some-url-here.ext/twtxt.txt") {
///               data
///           } else {
///               String::new()
///           };
/// ```
pub fn pull_twtxt(url: &str) -> Result<String> {
    let resp = ureq::get(&url).timeout_connect(5000).call();
    if resp.error() {
        return Err(Box::new(failure::Error::compat(format_err!(
            "{} :: {}",
            resp.status(),
            &url
        ))));
    }

    if let Ok(val) = resp.into_string() {
        return Ok(val);
    }
    Err(Box::new(failure::Error::compat(format_err!(
        "{} :: Internal Error",
        &url
    ))))
}

/// Wrapper to apply a function to each line of a `twtxt.txt` file,
/// returning the resulting lines as a `Vec<String>`
///
/// # Examples
/// ```
/// # use rustwtxt;
/// let input = "test\ntest";
/// let output = rustwtxt::mutate(input, |line| {
///         line.chars()
///             .map(|c| c.to_uppercase().to_string())
///             .collect::<String>()
///     });
/// assert_eq!("TEST", output[0]);
/// ```
pub fn mutate(twtxt: &str, f: fn(&str) -> String) -> Vec<String> {
    twtxt
        .to_owned()
        .lines()
        .map(|line| f(line))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://gbmor.dev/twtxt.txt";

    #[test]
    fn the_structs() {
        let twtxt = Twtxt::from(TEST_URL).unwrap();
        assert_eq!("gbmor", twtxt.nick());
        assert_eq!(TEST_URL, twtxt.url());
        assert!(twtxt.tweets().len() > 1);

        let (_, tweet) = twtxt.tweets().iter().next().unwrap();
        assert!(tweet.body().len() > 1);
        assert!(tweet.timestamp().len() > 1);
        assert!(tweet.tags().is_empty());
    }

    #[test]
    #[should_panic]
    fn bad_twtxt_url() {
        Twtxt::from("https://example.com/twtxt.txt").unwrap();
    }

    #[test]
    fn make_twtxt() {
        let rhs = Twtxt::from(TEST_URL).unwrap();
        let tweets = BTreeMap::new();
        let lhs = Twtxt {
            nickname: String::from("gbmor"),
            url: String::from("https://gbmor.dev/twtxt.txt"),
            tweets,
        };
        assert_eq!(lhs.nickname, rhs.nickname);
        assert_eq!(lhs.url, rhs.url);
        assert!(rhs.tweets.len() > 1);
    }

    #[test]
    fn test_mutate() {
        let input = "test";
        let rhs = mutate(input, |line| {
            line.chars()
                .map(|c| c.to_uppercase().to_string())
                .collect::<String>()
        });
        assert_eq!("TEST", rhs[0]);
    }

    #[test]
    fn test_pull_twtxt() {
        let res = pull_twtxt(TEST_URL).unwrap();
        assert!(res.contains("gbmor"));
    }

    #[test]
    #[should_panic]
    fn test_bad_url() {
        pull_twtxt("https://example-some-fake-site-goes-here.com/some_fake_url.txt").unwrap();
    }
}
