#![allow(dead_code)]

use std::collections::BTreeMap;

use http_req::request;
use regex::Regex;

pub mod parse;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Holds statuses and metadata from a single `twtxt.txt` file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Twtxt {
    nickname: String,
    url: String,
    tweets: BTreeMap<String, Tweet>,
}

impl Twtxt {
    pub fn nick<'a>(&'a self) -> &'a str {
        &self.nickname
    }
    pub fn url<'a>(&'a self) -> &'a str {
        &self.url
    }
    pub fn tweet(&self, datestamp: &str) -> Option<&Tweet> {
        if self.tweets.contains_key(datestamp) {
            Some(&self.tweets[datestamp])
        } else {
            None
        }
    }
    pub fn tweets(&self) -> &BTreeMap<String, Tweet> {
        &self.tweets
    }
    pub fn from(url: &str) -> Option<Self> {
        let twtxt = if let Ok(val) = pull_twtxt(&url) {
            val
        } else {
            return None;
        };
        let url = if let Ok(val) = parse::metadata(&twtxt, "url") {
            val.to_owned()
        } else {
            return None;
        };
        let nickname = if let Ok(val) = parse::metadata(&twtxt, "nick") {
            val.to_owned()
        } else {
            return None;
        };

        let mut tweets = BTreeMap::new();
        twtxt
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|line| {
                if line.starts_with("#") || line == &"" {
                    return;
                }
                let tweet = Tweet::from_str(line);
                tweets.insert(tweet.timestamp.clone(), tweet);
            });

        Some(Twtxt {
            nickname,
            url,
            tweets,
        })
    }
}

/// Holds a single status.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tweet {
    timestamp: String,
    body: String,
    mentions: Vec<String>,
    tags: Vec<String>,
}

impl Tweet {
    pub fn timestamp<'a>(&'a self) -> &'a str {
        &self.timestamp
    }
    pub fn body<'a>(&'a self) -> &'a str {
        &self.body
    }
    pub fn mentions(&self) -> Vec<String> {
        self.mentions.clone()
    }
    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }
    pub fn from_str(tweet: &str) -> Self {
        let split = tweet.split("\t").collect::<Vec<&str>>();
        let timestamp = split[0].to_string();
        let body = split[1].to_string();

        let mentions_regex = Regex::new(r"[@<].*[>]+").unwrap();
        let tags_regex = Regex::new(r"(^|\s)#[^\s]+").unwrap();

        let mentions = mentions_regex
            .find_iter(&body)
            .map(|ding| ding.as_str().to_string())
            .collect::<Vec<String>>();

        let tags = tags_regex
            .find_iter(&body)
            .map(|ding| {
                let tmp = ding.as_str();
                let tmp = tmp.split(" ").collect::<Vec<&str>>();
                if tmp[0] == "" && tmp.len() > 1 {
                    return tmp[1].to_string();
                }
                tmp[0].to_string()
            })
            .collect::<Vec<String>>();

        Tweet {
            timestamp,
            body,
            mentions,
            tags,
        }
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
    let mut buf = Vec::new();
    request::get(&url, &mut buf)?;
    let res = std::str::from_utf8(&buf)?.into();
    Ok(res)
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
        .map(|line| f(line).to_string())
        .collect::<Vec<String>>()
        .clone()
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
        assert!(tweet.tags().len() < 1);
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

    // This is causing cargo-tarpaulin to segfault, but
    // only in travis...
    #[ignore]
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
