//! Lower-level parsing functions for when you don't want to use
//! the provided `Twtxt` and `Tweet` objects.

use std::collections::BTreeMap;

use regex::Regex;

type StringErr<'a, T> = std::result::Result<T, &'a str>;

/// This parses out the specified information in the `== Metadata ==` section of
/// a given `twtxt.txt` file.
///
/// # Examples
/// ```
/// # use rustwtxt;
/// # use rustwtxt::parse;
/// let twtxt = rustwtxt::pull_twtxt("https://example.org/twtxt.txt").unwrap();
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

/// Pull the individual tweets from a remote `twtxt.txt` file into
/// a `std::collections::BTreeMap<String, String>`, The timestamp
/// is the key while the status is the value.
pub fn statuses(twtxt: &str) -> Option<BTreeMap<String, String>> {
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

    if map.len() == 0 {
        return None;
    }
    Some(map)
}

/// Parse the mentions out of a `twtxt.txt` file. Returns a
/// `std::collections::BTreeMap<String, String>` with the
/// timestamp of the tweet as the key and the mention as
/// the associated value.
pub fn mentions(twtxt: &str) -> Option<BTreeMap<String, String>> {
    let statuses = if let Some(val) = statuses(&twtxt) {
        val
    } else {
        return None;
    };
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

    if map.len() == 0 {
        return None;
    }
    Some(map)
}

/// Takes a mention in the form of `@<nick https://example.com/twtxt.txt>`
/// and reduces it to just the nickname.
///
/// # Examples
/// ```
/// # use rustwtxt;
/// # use rustwtxt::parse;
/// let status = "2019.09.09\tHey there, @<nickname https://example.com/twtxt.txt!>";
/// let mention = parse::mention_to_nickname(status).unwrap();
/// assert_eq!(mention, "nickname");
/// ```
pub fn mention_to_nickname(line: &str) -> Option<String> {
    let regex = Regex::new(r"[@<].*[>]+").unwrap();
    let mention = if let Some(val) = regex.captures(line) {
        match val.get(0) {
            Some(n) => n.as_str(),
            _ => return None,
        }
    } else {
        return None;
    };

    let mention_trimmed = mention[2..mention.len() - 1].to_string();
    let mention_split = mention_trimmed.split(" ").collect::<Vec<&str>>();
    Some(mention_split[0].into())
}

/// Parses out `#tags` from each tweet, returning a `std::collections::BTreeMap<String, String>`
/// with the timestamp as the key, and the tag as the value.
pub fn tags(twtxt: &str) -> Option<BTreeMap<String, String>> {
    let statuses = if let Some(val) = statuses(&twtxt) {
        val
    } else {
        return None;
    };
    let mut map = BTreeMap::new();
    statuses.iter().for_each(|(k, v)| {
        if !v.contains("#") {
            return;
        }

        let regex = Regex::new(r"(^|\s)#[^\s]+").unwrap();
        let tag: Vec<(String, String)> = regex
            .find_iter(v)
            .map(|ding| (k.clone(), ding.as_str().to_string()))
            .collect();

        let tags: Vec<(String, String)> = tag
            .iter()
            .map(|(k, v)| {
                let v = v
                    .chars()
                    .map(|c| {
                        if c.is_whitespace() {
                            return "".into();
                        }
                        c.to_string()
                    })
                    .collect::<String>();
                (k.clone(), v)
            })
            .collect();

        let mut tag_group = String::new();
        tags.iter().for_each(|(_, v)| {
            tag_group.push_str(v);
            tag_group.push_str(" ");
        });

        map.insert(k.to_string(), tag_group[..tag_group.len() - 1].to_string());
    });

    if map.len() == 0 {
        return None;
    }
    Some(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://gbmor.dev/twtxt.txt";

    #[test]
    fn turn_mentions_to_nick() {
        let twtxt = "2019.09.09\tHey @<gbmor https://gbmor.dev/twtxt.txt>!";
        let mention = mention_to_nickname(twtxt).unwrap();
        assert_eq!("gbmor", mention);
    }

    #[test]
    fn get_tags() {
        let tag_map = tags("test\t#test").unwrap();
        assert!("#test" == &tag_map["test"]);

        let tag_map = tags("test\tsome other #test here").unwrap();
        assert!("#test" == &tag_map["test"]);

        let tag_map = tags("test\tsome other #test").unwrap();
        assert!("#test" == &tag_map["test"]);

        let tag_map = tags("test\tsome #test goes #here").unwrap();
        assert!("#test #here" == &tag_map["test"]);
    }

    #[test]
    #[should_panic]
    fn bad_regex() {
        metadata("SOME DATA", "<#*#@(&$(%)@$)>").unwrap();
    }

    #[test]
    #[should_panic]
    fn no_matches() {
        metadata("SOME = DATA", "nick").unwrap();
    }

    #[test]
    fn get_mentions() {
        let twtxt = crate::pull_twtxt(TEST_URL).unwrap();
        let mention_map = mentions(&twtxt).unwrap();
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
        let res = statuses(&twtxt).unwrap();
        assert!(res.len() > 1);
    }
    #[test]
    #[should_panic]
    fn parse_bad_twtxt() {
        metadata("SOMETHING GOES HERE", "url").unwrap();
    }

    #[test]
    #[should_panic]
    fn get_bad_statuses() {
        statuses("").unwrap();
    }
}
