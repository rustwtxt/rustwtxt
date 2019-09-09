#![feature(try_trait)]
#![allow(dead_code)]

use std::time::Duration;

use reqwest;

pub mod parse;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn build_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(10))
        .build()?;
    Ok(client)
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
    let client = build_client()?;
    let res = client.get(url).send()?.text()?;
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
    fn test_build_client() {
        // Right now, just panic if it returns Err()
        build_client().unwrap();
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
