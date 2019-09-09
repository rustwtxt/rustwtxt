#![allow(dead_code)]

use std::collections::BTreeMap;

use http_req::request;

pub mod parse;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Holds statuses and metadata from a single `twtxt.txt` file.
#[derive(Debug, Clone, Eq, PartialEq)]
struct Twtxt {
    nickname: String,
    url: String,
    tweets: BTreeMap<String, Tweet>,
}

/// Holds a single status.
#[derive(Debug, Clone, Eq, PartialEq)]
struct Tweet {
    timestamp: String,
    body: String,
    mentions: Vec<String>,
    tags: Vec<String>,
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
