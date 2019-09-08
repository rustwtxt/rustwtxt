#![allow(dead_code)]

use std::time::Duration;

use reqwest;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn build_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(10))
        .build()?;
    Ok(client)
}

pub fn pull_twtxt(url: &str) -> Result<String> {
    let client = build_client()?;
    let res = client.get(url).send()?.text()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_client() {
        // Right now, just panic if it returns Err()
        build_client().unwrap();
    }
    #[test]
    fn test_pull_twtxt() {
        let res = pull_twtxt("https://gbmor.dev/twtxt.txt").unwrap();
        eprintln!("{}", res);
        assert!(res.contains("gbmor"));
    }
}
