use std::collections::BTreeMap;

use regex::Regex;

type StringErr<'a, T> = std::result::Result<T, &'a str>;

/// This parses out the specified information in the `== Metadata ==` section of
/// a given `twtxt.txt` file.
///
/// # Examples
/// ```
/// # use rustwtxt::*;
/// # use rustwtxt::parse;
/// let twtxt = pull_twtxt("https://example.org/twtxt.txt").unwrap();
/// let out = parse::metadata(&twtxt, "nick");
/// ```
pub fn metadata<'a, 'b>(twtxt: &'a str, keyword: &'b str) -> StringErr<'a, &'a str> {
    if !twtxt.contains("== Metadata ==") || !twtxt.contains(keyword) {
        return Err("File contains no metadata section, or the keyword is missing");
    }

    let regex_string = format!("{} = (.*)", keyword);

    let regex = if let Ok(val) = Regex::new(&regex_string) {
        val
    } else {
        return Err("No Keyword Matches");
    };

    let matched = if let Some(val) = regex.captures(twtxt) {
        val
    } else {
        return Err("No Keyword Matches");
    };

    let keyword_match = if let Some(val) = matched.get(1) {
        val.as_str()
    } else {
        return Err("Keyword Matched Out of Bounds");
    };

    Ok(keyword_match)
}

/// Pull the individual statuses from a remote `twtxt.txt` file into
/// a `std::collections::BTreeMap<String, String>`, The timestamp
/// is the key while the status is the value.
pub fn statuses(twtxt: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let lines = twtxt.split("\n").collect::<Vec<&str>>();
    lines.iter().for_each(|line| {
        if line.starts_with("#") || line.len() < 1 {
            return;
        }

        let status = line.split("\t").collect::<Vec<&str>>();
        let datestamp = status[0];
        map.insert(datestamp.into(), status[1].into());
    });
    map
}

/// Parse the mentions out of a `twtxt.txt` file. Returns a
/// `std::collections::BTreeMap<String, String>` with the
/// timestamp of the status as the key and the mention as
/// the associated value.
pub fn mentions(twtxt: &str) -> BTreeMap<String, String> {
    let statuses = statuses(&twtxt);
    let mut map = BTreeMap::new();
    statuses.iter().for_each(|(k, v)| {
        if !v.contains("@<") {
            return;
        }

        let regex = Regex::new(r"[@<].*[>]+").unwrap();
        let out = if let Some(val) = regex.captures(v) {
            match val.get(0) {
                Some(n) => n.as_str(),
                _ => return,
            }
        } else {
            return;
        };

        let mention = out[2..out.len() - 1].to_string();
        map.insert(k.to_string(), mention);
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://gbmor.dev/twtxt.txt";

    #[test]
    fn get_mentions() {
        let twtxt = crate::pull_twtxt(TEST_URL).unwrap();
        let mention_map = mentions(&twtxt);
        assert!(mention_map.len() > 1);
    }

    #[test]
    fn get_username() {
        let res = crate::pull_twtxt(TEST_URL).unwrap();
        let user = metadata(&res, "nick").unwrap();
        assert_eq!("gbmor", user);
    }

    // This passes `cargo test`, but `cargo tarpaulin` segfaults
    #[ignore]
    #[test]
    fn get_url() {
        let res = crate::pull_twtxt(TEST_URL).unwrap();
        let url = metadata(&res, "url").unwrap();
        assert_eq!(TEST_URL, url);
    }

    #[test]
    fn get_status_map() {
        let twtxt = crate::pull_twtxt(TEST_URL).unwrap();
        let res = statuses(&twtxt);
        assert!(res.len() > 1);
    }
    #[test]
    #[should_panic]
    fn parse_bad_twtxt() {
        metadata("SOMETHING GOES HERE", "url").unwrap();
    }

    #[test]
    fn get_bad_statuses() {
        let status_map = statuses("");
        assert!(status_map.len() < 1);
    }
}
