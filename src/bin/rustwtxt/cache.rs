type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_remote_modtime(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let resp = client.head(url).send()?;
    let headers = resp.headers();

    if headers.contains_key("Last-Modified") {
        match headers.get("Last-Modified") {
            Some(val) => return Ok(val.to_str()?.into()),
            None => return Err("Last-Modified Header Empty".into()),
        };
    }

    Err("Last-Modified Not Found in Response Headers".into())
}
