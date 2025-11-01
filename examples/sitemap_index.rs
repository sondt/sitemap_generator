use sitemap_generator::{SitemapIndexBuilder, SitemapIndexEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sitemap index builder
    let mut builder = SitemapIndexBuilder::new();

    // Add sitemap entries
    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/sitemap1.xml.gz")
            .lastmod("2025-11-01")
    );

    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/sitemap2.xml.gz")
            .lastmod("2025-11-02")
    );

    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/image-sitemap.xml.gz")
            .lastmod("2025-11-03")
    );

    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/video-sitemap.xml.gz")
            .lastmod("2025-11-04")
    );

    // Generate the XML
    let xml = builder.build()?;
    println!("Generated sitemap index ({} bytes):", xml.len());
    println!("{}", xml);

    // Write to file
    builder.write("sitemap_index.xml")?;
    println!("\nSitemap index written to sitemap_index.xml");

    Ok(())
}
