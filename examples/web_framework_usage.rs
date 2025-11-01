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

    // Example 1: Axum
    println!("--- Axum Example ---");
    println!(r#"
use axum::{{
    response::{{Response, IntoResponse}},
    http::{{StatusCode, header}},
}};
use sitemap_generator::{{SitemapBuilder, UrlEntry}};

async fn sitemap() -> impl IntoResponse {{
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    match builder.build_bytes() {{
        Ok(bytes) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(bytes.into())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new().into())
            .unwrap(),
    }}
}}

// With compression:
async fn sitemap_compressed() -> impl IntoResponse {{
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    match builder.build_compressed_bytes() {{
        Ok(bytes) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/xml")
            .header(header::CONTENT_ENCODING, "gzip")
            .body(bytes.into())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new().into())
            .unwrap(),
    }}
}}
"#);

    // Example 2: Actix-web
    println!("\n--- Actix-web Example ---");
    println!(r#"
use actix_web::{{web, HttpResponse, Result}};
use sitemap_generator::{{SitemapBuilder, UrlEntry}};

async fn sitemap() -> Result<HttpResponse> {{
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    match builder.build_bytes() {{
        Ok(bytes) => Ok(HttpResponse::Ok()
            .content_type("application/xml; charset=utf-8")
            .body(bytes)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }}
}}

// With compression:
async fn sitemap_compressed() -> Result<HttpResponse> {{
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    match builder.build_compressed_bytes() {{
        Ok(bytes) => Ok(HttpResponse::Ok()
            .content_type("application/xml")
            .insert_header(("Content-Encoding", "gzip"))
            .body(bytes)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }}
}}
"#);

    // Example 3: Rocket
    println!("\n--- Rocket Example ---");
    println!(r#"
use rocket::{{State, response::content::RawXml}};
use sitemap_generator::{{SitemapBuilder, UrlEntry}};

#[get("/sitemap.xml")]
fn sitemap() -> Option<RawXml<Vec<u8>>> {{
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    builder.build_bytes()
        .ok()
        .map(RawXml)
}}
"#);

    // Example 4: Warp
    println!("\n--- Warp Example ---");
    println!(r#"
use warp::{{Filter, reply, http::Response}};
use sitemap_generator::{{SitemapBuilder, UrlEntry}};

fn sitemap() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {{
    warp::path("sitemap.xml")
        .map(|| {{
            let mut builder = SitemapBuilder::new();
            builder.add_url(UrlEntry::new("https://example.com/"));

            match builder.build_bytes() {{
                Ok(bytes) => Response::builder()
                    .header("Content-Type", "application/xml; charset=utf-8")
                    .body(bytes)
                    .unwrap(),
                Err(_) => Response::builder()
                    .status(500)
                    .body(Vec::new())
                    .unwrap(),
            }}
        }})
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
