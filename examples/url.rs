use std::{net::IpAddr, str::FromStr};

use serde::Serialize;
use url::Url;
use valitron::{custom, Validator};

pub fn main() {
    let validator = Validator::new()
        .rule("url", custom(valid_url))
        .rule("ip", custom(valid_ip));

    let data = Data {
        url: "http://example.com".into(),
        ip: "127.0.0.1".into(),
    };

    let _ = validator.validate(&data);
}

#[derive(Debug, Serialize)]
struct Data {
    url: String,
    ip: String,
}

fn valid_url(s: &mut String) -> Result<(), String> {
    Url::parse(s)
        .map_err(|_| "error parsing url".into())
        .map(|_| ())
}

fn valid_ip(ip: &mut String) -> Result<(), String> {
    IpAddr::from_str(ip)
        .map(|_| ())
        .map_err(|_| "error parsing ip".into())
}
