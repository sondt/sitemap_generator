use sitemap_generator::{SitemapBuilder, UrlEntry, ChangeFreq};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new sitemap builder
    let mut builder = SitemapBuilder::new();

    // Add URLs to the sitemap
    builder.add_url(
        UrlEntry::new("https://example.com/")
            .lastmod("2025-11-01")
            .changefreq(ChangeFreq::Daily)
            .priority(1.0)
    );

    builder.add_url(
        UrlEntry::new("https://example.com/about")
            .lastmod("2025-11-02")
            .changefreq(ChangeFreq::Monthly)
            .priority(0.8)
    );

    builder.add_url(
        UrlEntry::new("https://example.com/products")
            .lastmod("2025-11-03")
            .changefreq(ChangeFreq::Weekly)
            .priority(0.9)
    );

    builder.add_url(
        UrlEntry::new("https://example.com/contact")
            .lastmod("2025-11-04")
            .priority(0.7)
    );

    // Generate the XML
    let xml = builder.build()?;
    println!("Generated sitemap ({} bytes):", xml.len());
    println!("{}", xml);

    // Write to file
    builder.write("sitemap.xml")?;
    println!("\nSitemap written to sitemap.xml");

    // Write compressed version
    builder.write_compressed("sitemap.xml.gz")?;
    println!("Compressed sitemap written to sitemap.xml.gz");

    Ok(())
}
