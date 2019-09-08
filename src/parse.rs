use std::collections::BTreeMap;

use chrono::prelude::*;

/// This parses out the information in the "== Metadata ==" section of
/// a given `twtxt.txt` file.
/// You'll have to play with the trim offset here. Often there will be
/// unprintable characters, such as newlines, appended despite the output
/// being `trim()`-ed before return.
///
/// # Examples
/// ```
/// # use rustwtxt::*;
/// # use rustwtxt::parse;
/// let twtxt = pull_twtxt("https://example.org/twtxt.txt").unwrap();
/// let out = parse::metadata(&twtxt, "nick", 1);
/// ```
pub fn metadata(twtxt: &str, keyword: &str, trim: usize) -> String {
    if !twtxt.contains("== Metadata ==") || !twtxt.contains(keyword) {
        return String::new();
    }

    let split_at_keyword = twtxt.split(keyword).collect::<Vec<&str>>();
    let rhs_of_keyword = split_at_keyword[1];
    let split_at_equals = rhs_of_keyword.split("=").collect::<Vec<&str>>();
    let rhs_of_equals = split_at_equals[1];
    let split_at_space = rhs_of_equals.split(" ").collect::<Vec<&str>>();
    let word_untrimmed = split_at_space[1];
    let word_len = word_untrimmed.len();
    let word = word_untrimmed[..word_len - trim].to_string();
    word.trim().into()
}

/// Pull the individual statuses from a remote `twtxt.txt` file into
/// a `std::collections::BTreeMap`, with the dates parsed into a
/// `chrono::DateTime<FixedOffset>`
///
/// # Examples
/// ```
///# use rustwtxt::*;
///# use rustwtxt::parse;
/// let twtxt = pull_twtxt("https://example.org/twtxt.txt").unwrap();
/// let status_map = parse::statuses(&twtxt);
/// ```
pub fn statuses(twtxt: &str) -> BTreeMap<chrono::DateTime<FixedOffset>, String> {
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
    fn test_get_username() {
        let res = crate::pull_twtxt(TEST_URL).unwrap();
        let user = metadata(&res, "nick", 1);
        assert_eq!("gbmor", &user);
    }

    // This passes `cargo test`, but `cargo tarpaulin` segfaults
    #[ignore]
    #[test]
    fn test_get_url() {
        let res = crate::pull_twtxt(TEST_URL).unwrap();
        let url = metadata(&res, "url", 4);
        assert_eq!(TEST_URL, &url);
    }

    #[test]
    fn test_status_map() {
        let twtxt = crate::pull_twtxt(TEST_URL).unwrap();
        let res = statuses(&twtxt);
        eprintln!("{:#?}", res);
        assert!(res.len() > 1);
    }
    #[test]
    fn parse_bad_twtxt() {
        let rhs = metadata("SOMETHING GOES HERE", "url", 0);
        assert_eq!(String::new(), rhs);
    }

    #[test]
    fn get_bad_statuses() {
        let out = statuses("2019-04-50\tSOME STATUS");
        assert!(out.len() < 1);
    }
}
