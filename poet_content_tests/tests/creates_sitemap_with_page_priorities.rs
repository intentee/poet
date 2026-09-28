use poet_content::create_sitemap::create_sitemap;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use sitemap_rs::jiff::Timestamp;
use sitemap_rs::jiff::tz::TimeZone;

#[test]
fn creates_sitemap_with_page_priorities() -> Result<(), PoetContentTestsError> {
    let home = fixture_reference(
        "index",
        "description = \"d\"\nlayout = \"L\"\ntitle = \"Home\"",
    )?;
    let page = fixture_reference(
        "docs/guide",
        "description = \"d\"\nlayout = \"L\"\ntitle = \"Guide\"",
    )?;

    assert_eq!(
        create_sitemap(
            &Timestamp::UNIX_EPOCH.to_zoned(TimeZone::UTC),
            [&home, &page].into_iter()
        )?,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\t<url>\n\t\t<loc>/</loc>\n\t\t<lastmod>1970-01-01T00:00:00+00:00</lastmod>\n\t\t<priority>0.8</priority>\n\t</url>\n\t<url>\n\t\t<loc>/docs/guide/</loc>\n\t\t<lastmod>1970-01-01T00:00:00+00:00</lastmod>\n\t\t<priority>0.5</priority>\n\t</url>\n</urlset>\n"
    );

    Ok(())
}
