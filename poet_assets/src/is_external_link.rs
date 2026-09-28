use url::Url;

const SITE_BASE_URL: &str = "https://poet.invalid/";

#[must_use]
pub fn is_external_link(link: &str) -> bool {
    Url::parse(SITE_BASE_URL)
        .and_then(|site_base_url| {
            site_base_url
                .join(link)
                .map(|resolved_link| resolved_link.origin() != site_base_url.origin())
        })
        .unwrap_or(true)
}
