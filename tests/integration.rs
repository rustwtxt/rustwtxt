use rustwtxt;
use rustwtxt::parse;

#[test]
fn end_to_end() {
    let twtxt = rustwtxt::pull_twtxt("https://gbmor.dev/twtxt.txt").unwrap();
    let user = parse::metadata(&twtxt, "nick").unwrap();
    assert_eq!(user, "gbmor");

    let statuses = parse::statuses(&twtxt);
    assert!(statuses.len() > 1);
}
