use std::str::FromStr;

use anyhow::{Error, Result};
use robots_txt::Robots;
use url::Url;

pub fn generate_robots(base_url: String) -> Result<String, Error> {
    let base_url = Url::from_str(&base_url)?.to_string();

    let robots = Robots::builder()
        .start_section("*")
        .allow("/")
        .disallow("")
        .crawl_delay(3.0)
        .sitemap(Url::from_str(&format!("{base_url}sitemap.xml"))?)
        .end_section()
        .build();

    Ok(robots.to_string())
}
