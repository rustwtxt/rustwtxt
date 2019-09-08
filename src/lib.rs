#![allow(dead_code)]

use std::collections::BTreeMap;
use std::time::Duration;

use chrono::prelude::*;
use reqwest;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn build_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(10))
        .build()?;
    Ok(client)
}

/// Pulls the actual twtxt.txt file from the specified URL.
pub fn pull_twtxt(url: &str) -> Result<String> {
    let client = build_client()?;
    let res = client.get(url).send()?.text()?;
    Ok(res)
}

// You'll have to play with the trim offset here. Often there will be
// unprintable characters, such as newlines, appended despite the output
// being trim()-ed before return.
pub fn parse_metadata(twtxt: &str, keyword: &str, trim: usize) -> String {
    if !twtxt.contains("== Metadata ==") || !twtxt.contains(keyword) {
        return String::new();
    }

    let split = twtxt.split(keyword).collect::<Vec<&str>>();
    let split = split[1];
    let split = split.split("=").collect::<Vec<&str>>();
    let split = split[1];
    let split = split.split(" ").collect::<Vec<&str>>();
    let word = split[1];
    let word = word[..word.len() - trim].to_string();
    word.trim().into()
}

pub fn get_statuses(twtxt: &str) -> BTreeMap<chrono::DateTime<FixedOffset>, String> {
    let mut map = BTreeMap::<chrono::DateTime<FixedOffset>, String>::new();
    let lines = twtxt.split("\n").collect::<Vec<&str>>();
    lines.iter().for_each(|line| {
        if line.starts_with("#") || line.len() < 1 {
            return;
        }

        let status = line.split("\t").collect::<Vec<&str>>();
        let datestamp = if let Ok(val) = DateTime::parse_from_rfc3339(status[0]) {
            val
        } else {
            return;
        };
        map.insert(datestamp, status[1].into());
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://gbmor.dev/twtxt.txt";

    #[test]
    fn test_build_client() {
        // Right now, just panic if it returns Err()
        build_client().unwrap();
    }
    #[test]
    fn test_pull_twtxt() {
        let res = pull_twtxt(TEST_URL).unwrap();
        eprintln!("{}", res);
        assert!(res.contains("gbmor"));
    }

    #[test]
    fn test_get_username() {
        let res = pull_twtxt(TEST_URL).unwrap();
        let user = parse_metadata(&res, "nick", 1);
        assert_eq!("gbmor", &user);
    }

    #[test]
    fn test_get_url() {
        let res = pull_twtxt(TEST_URL).unwrap();
        let url = parse_metadata(&res, "url", 4);
        assert_eq!(TEST_URL, &url);
    }

    #[test]
    fn test_status_map() {
        let twtxt = pull_twtxt(TEST_URL).unwrap();
        let res = get_statuses(&twtxt);
        eprintln!("{:#?}", res);
        assert!(res.len() > 1);
    }

    #[test]
    #[should_panic]
    fn test_bad_url() {
        pull_twtxt("https://example-some-fake-site-goes-here.com/some_fake_url.txt").unwrap();
    }

    #[test]
    fn parse_bad_twtxt() {
        let rhs = parse_metadata("SOMETHING GOES HERE", "url", 0);
        assert_eq!(String::new(), rhs);
    }
}
