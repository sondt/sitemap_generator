//! Example demonstrating how to use sitemap_generator with web frameworks
//! This shows the pattern for Axum, Actix-web, and other frameworks

use sitemap_generator::{SitemapBuilder, UrlEntry, ChangeFreq};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Web Framework Usage Examples ===\n");

    // Create a sitemap
    let mut builder = SitemapBuilder::new();

    builder.add_url(
        UrlEntry::new("https://example.com/")
            .lastmod("2025-11-01")
            .changefreq(ChangeFreq::Daily)
            .priority(1.0)
    );

    builder.add_url(
        UrlEntry::new("https://example.com/about")
            .lastmod("2025-11-02")
            .priority(0.8)
    );

    // 1. Get as String (for frameworks that accept String)
    println!("1. As String:");
    let xml_string = builder.build()?;
    println!("   Size: {} bytes", xml_string.len());
    println!("   Type: String\n");

    // 2. Get as bytes (Vec<u8>) - for most HTTP responses
    println!("2. As Bytes (Vec<u8>):");
    let bytes = builder.build_bytes()?;
    println!("   Size: {} bytes", bytes.len());
    println!("   Type: Vec<u8>\n");

    // 3. Get as compressed bytes (gzip)
    println!("3. As Compressed Bytes (gzip):");
    let compressed = builder.build_compressed_bytes()?;
    println!("   Original size: {} bytes", bytes.len());
    println!("   Compressed size: {} bytes", compressed.len());
    println!("   Compression ratio: {:.1}x\n", bytes.len() as f64 / compressed.len() as f64);

    println!("=== Framework Integration Examples ===\n");

    // Example with Axum
    println!("--- Axum Example ---");
    println!(r#"
use axum::{{
    response::{{Response, IntoResponse}},
    http::{{StatusCode, header}},
    body::Body,
}};
use sitemap_generator::{{SitemapIndexBuilder, SitemapIndexEntry}};

async fn sitemap() -> impl IntoResponse {{
    let mut builder = SitemapBuilder::new();
    builder.add_sitemap(SitemapIndexEntry::new("https://example.com/));

    match builder.build_bytes() {{
        Ok(bytes) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/xml")
            .body(Body::from(bytes))
            .unwrap(),
        Err(e) => {{
            println!("Error generating sitemap index: {{}}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(Vec::<u8>::new()))
                .unwrap()
        }}
    }}
}}

"#);


    println!("\n=== Tips ===");
    println!("1. Use build_bytes() for standard responses");
    println!("2. Use build_compressed_bytes() for better performance (smaller size)");
    println!("3. Always set Content-Type: application/xml");
    println!("4. Add Content-Encoding: gzip when using compressed bytes");
    println!("5. Cache the sitemap in production to avoid regeneration on each request");

    Ok(())
}
