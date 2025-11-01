use sitemap_generator::{SitemapBuilder, UrlEntry, ChangeFreq};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Compression Performance Test ===\n");

    // Test with different sizes
    let sizes = vec![10, 100, 1000, 10000];

    for size in sizes {
        let mut builder = SitemapBuilder::new();

        for i in 0..size {
            builder.add_url(
                UrlEntry::new(format!("https://example.com/page/{}", i))
                    .lastmod("2025-11-01")
                    .changefreq(ChangeFreq::Daily)
                    .priority(0.8)
            );
        }

        let bytes = builder.build_bytes()?;
        let compressed = builder.build_compressed_bytes()?;

        println!("{:>6} URLs:", size);
        println!("  Uncompressed:    {:>8} bytes", bytes.len());
        println!("  Compressed:      {:>8} bytes", compressed.len());
        println!("  Ratio:           {:>8.2}x", bytes.len() as f64 / compressed.len() as f64);
        println!("  Bandwidth saved: {:>7.1}%", (1.0 - compressed.len() as f64 / bytes.len() as f64) * 100.0);
        println!();
    }

    println!("💡 Tip: Use build_compressed_bytes() for HTTP responses to save bandwidth!");

    Ok(())
}
