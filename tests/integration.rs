use rustwtxt;
use rustwtxt::parse;

#[ignore]
#[test]
fn end_to_end() {
    let twtxt = rustwtxt::pull_twtxt("https://gbmor.dev/twtxt.txt").unwrap();
    let user = parse::metadata(&twtxt, "nick").unwrap();
    assert_eq!(user, "gbmor");

    let statuses = parse::statuses(&twtxt).unwrap();
    assert!(statuses.len() > 1);

    let mentions = parse::mentions(&twtxt).unwrap();
    assert!(mentions.len() > 1);

    let mention = "@<nick url>";
    let mention_nick = parse::mention_to_nickname(&mention).unwrap();
    assert_eq!("nick", mention_nick);

    let twtxt = rustwtxt::Twtxt::from("https://gbmor.dev/twtxt.txt").unwrap();
    let tweets = twtxt.tweets();

    let (_, tweet) = tweets.iter().next().unwrap();
    assert!(tweet.body().len() > 1);
    assert!(twtxt.nick() == "gbmor");
    assert!(twtxt.url() == "https://gbmor.dev/twtxt.txt");
}
