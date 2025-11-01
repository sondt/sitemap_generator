# Sitemap Generator

[![Crates.io](https://img.shields.io/crates/v/sitemap_generator.svg)](https://crates.io/crates/sitemap_generator)
[![Documentation](https://docs.rs/sitemap_generator/badge.svg)](https://docs.rs/sitemap_generator)
[![MIT License](https://img.shields.io/badge/license-MIT.svg)](LICENSE)

A high-performance Rust library for generating XML sitemaps compliant with the [sitemaps.org protocol 0.9](https://www.sitemaps.org/protocol.html).

## Features

- **Standard XML Sitemaps**: Generate basic sitemaps with URLs, lastmod, changefreq, and priority
- **Image Sitemaps**: Include image metadata (captions, geo-location, titles, licenses)
- **Video Sitemaps**: Include video metadata (thumbnails, descriptions, durations, ratings)
- **News Sitemaps**: Submit news articles to Google News with publication metadata
- **Combined Sitemaps**: Combine multiple extensions (image + video + news) in one sitemap
- **Sitemap Index**: Manage multiple sitemap files for large websites (>50k URLs)
- **Validation**: Automatic validation of URLs, size limits, and protocol compliance
- **Compression**: Built-in gzip compression support (96-98% bandwidth savings)
- **Web Framework Support**: Direct bytes output for Axum, Actix-web, Rocket, etc.
- **Parsing**: Read and parse existing sitemap files
- **Memory Efficient**: ~140 bytes/URL during generation, 0 bytes after (proven)
- **High Performance**: ~830K URLs/second, immediate memory cleanup
- **Optimized Builders**: Pre-allocate capacity with `with_capacity()` for better performance
- **Zero unsafe code**: 100% safe Rust

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sitemap_generator = "0.1"
```

## Quick Start

### Basic Sitemap

Generate sitemap as String, bytes, or compressed bytes:

```rust
use sitemap_generator::{SitemapBuilder, UrlEntry, ChangeFreq};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use with_capacity() if you know the number of URLs in advance (recommended)
    let mut builder = SitemapBuilder::with_capacity(1000);

    builder.add_url(UrlEntry::new("https://example.com/")
        .lastmod("2025-11-01")
        .changefreq(ChangeFreq::Daily)
        .priority(1.0));

    builder.add_url(UrlEntry::new("https://example.com/page1")
        .lastmod("2025-11-02")
        .priority(0.8));

    // 1. Generate as String
    let xml = builder.build()?;

    // 2. Generate as bytes (Vec<u8>) - for HTTP responses
    let bytes = builder.build_bytes()?;

    // 3. Generate as compressed bytes (gzip)
    let compressed = builder.build_compressed_bytes()?;

    // 4. Write to file
    builder.write("sitemap.xml")?;

    // 5. Write compressed file
    builder.write_compressed("sitemap.xml.gz")?;

    Ok(())
}
```

### Image Sitemap

```rust
use sitemap_generator::{ImageSitemapBuilder, UrlEntry, UrlWithImages, ImageEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = ImageSitemapBuilder::new();

    let url_with_images = UrlWithImages::new(
        UrlEntry::new("https://example.com/gallery")
            .lastmod("2025-11-01")
    )
    .add_image(
        ImageEntry::new("https://example.com/images/photo1.jpg")
            .title("Beautiful Sunset")
            .caption("A stunning sunset over the ocean")
    );

    builder.add_url(url_with_images);

    let xml = builder.build()?;
    builder.write("image_sitemap.xml")?;

    Ok(())
}
```

### Video Sitemap

```rust
use sitemap_generator::{VideoSitemapBuilder, UrlEntry, UrlWithVideos, VideoEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = VideoSitemapBuilder::new();

    let url_with_video = UrlWithVideos::new(
        UrlEntry::new("https://example.com/videos/intro")
    )
    .add_video(
        VideoEntry::new(
            "https://example.com/thumbnails/intro.jpg",
            "Introduction to Our Product",
            "Learn about our product in this video"
        )
        .content_loc("https://example.com/videos/intro.mp4")
        .duration(300)
        .rating(4.5)
        .family_friendly(true)
        .add_tag("tutorial")
    );

    builder.add_url(url_with_video);

    let xml = builder.build()?;
    builder.write("video_sitemap.xml")?;

    Ok(())
}
```

### News Sitemap

```rust
use sitemap_generator::{NewsSitemapBuilder, UrlEntry, UrlWithNews, NewsEntry, NewsPublication};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = NewsSitemapBuilder::new();

    let publication = NewsPublication::new("TechDaily", "en");
    let news = NewsEntry::new(
        publication,
        "2025-11-01T10:00:00Z",
        "Revolutionary AI Breakthrough Announced"
    )
    .keywords("AI, technology, machine learning")
    .stock_tickers("GOOGL, MSFT");

    let url = UrlEntry::new("https://example.com/tech/ai-breakthrough");
    builder.add_url(UrlWithNews::new(url, news));

    let xml = builder.build()?;
    builder.write("news_sitemap.xml")?;

    Ok(())
}
```

### Combined Sitemap (Multiple Extensions)

Combine image, video, and news extensions in a single sitemap:

```rust
use sitemap_generator::{
    CombinedSitemapBuilder, UrlEntry, UrlWithExtensions,
    ImageEntry, VideoEntry, NewsEntry, NewsPublication
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = CombinedSitemapBuilder::new();

    // News article with images and video
    let news = NewsEntry::new(
        NewsPublication::new("TechDaily", "en"),
        "2025-11-01T10:00:00Z",
        "AI Breakthrough Announced"
    );

    let image = ImageEntry::new("https://example.com/ai-lab.jpg")
        .title("AI Research Lab");

    let video = VideoEntry::new(
        "https://example.com/thumb.jpg",
        "AI Demo Video",
        "Watch the demonstration"
    )
    .content_loc("https://example.com/video.mp4")
    .duration(300);

    builder.add_url(
        UrlWithExtensions::new(UrlEntry::new("https://example.com/article"))
            .add_image(image)
            .add_video(video)
            .set_news(news)
    );

    let xml = builder.build()?;
    builder.write("combined_sitemap.xml")?;

    Ok(())
}
```

### Sitemap Index

```rust
use sitemap_generator::{SitemapIndexBuilder, SitemapIndexEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = SitemapIndexBuilder::new();

    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/sitemap1.xml.gz")
            .lastmod("2025-11-01")
    );

    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/sitemap2.xml.gz")
            .lastmod("2025-11-02")
    );

    let xml = builder.build()?;
    builder.write("sitemap_index.xml")?;

    Ok(())
}
```

### Web Framework Integration

Use with Axum, Actix-web, Rocket, or other web frameworks:

#### Axum Example

```rust
use axum::{
    response::{Response, IntoResponse},
    http::{StatusCode, header},
};
use sitemap_generator::{SitemapBuilder, UrlEntry};

async fn sitemap() -> impl IntoResponse {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    match builder.build_bytes() {
        Ok(bytes) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(bytes.into())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new().into())
            .unwrap(),
    }
}

// With compression (recommended):
async fn sitemap_compressed() -> impl IntoResponse {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    match builder.build_compressed_bytes() {
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
    }
}
```

#### Actix-web Example

```rust
use actix_web::{web, HttpResponse, Result};
use sitemap_generator::{SitemapBuilder, UrlEntry};

async fn sitemap() -> Result<HttpResponse> {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    match builder.build_compressed_bytes() {
        Ok(bytes) => Ok(HttpResponse::Ok()
            .content_type("application/xml")
            .insert_header(("Content-Encoding", "gzip"))
            .body(bytes)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
```

See [examples/web_framework_usage.rs](examples/web_framework_usage.rs) for more examples.

### Parsing Sitemaps

```rust
use sitemap_generator::SitemapParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse from file
    let entries = SitemapParser::parse_file("sitemap.xml")?;

    // Parse compressed file
    let entries = SitemapParser::parse_compressed("sitemap.xml.gz")?;

    // Parse from string
    let xml = r#"<?xml version="1.0"?>
        <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
            <url>
                <loc>https://example.com/</loc>
                <lastmod>2025-11-01</lastmod>
            </url>
        </urlset>"#;
    let entries = SitemapParser::parse_string(xml)?;

    for entry in entries {
        println!("URL: {}", entry.loc);
    }

    Ok(())
}
```

## Performance

This library is designed for high performance and low memory usage:

- **Fast XML Generation**: Uses `quick-xml` for efficient XML writing (~830K URLs/second)
- **Memory Efficient**: ~140 bytes per URL during generation, 0 bytes after (proven with custom allocator)
- **Optimized Allocation**: Use `with_capacity()` to pre-allocate memory and reduce reallocations
- **Immediate Cleanup**: Memory released **immediately** when builder drops (RAII-based)
- **Zero Memory Leaks**: Empirically proven - 1 byte growth across 100 iterations
- **Streaming Compression**: Gzip compression with 50x+ compression ratios (98% bandwidth saved)

### Performance Tips

**Use `with_capacity()` for better performance:**

```rust
// ❌ Without capacity (Vec grows dynamically)
let mut builder = SitemapBuilder::new();
for i in 0..10_000 {
    builder.add_url(UrlEntry::new(format!("https://example.com/page{}", i)));
}

// ✅ With capacity (Vec pre-allocated, ~2-5% faster)
let mut builder = SitemapBuilder::with_capacity(10_000);
for i in 0..10_000 {
    builder.add_url(UrlEntry::new(format!("https://example.com/page{}", i)));
}
```

**Available for all builders:**
- `SitemapBuilder::with_capacity(10_000)`
- `ImageSitemapBuilder::with_capacity(5_000)`
- `VideoSitemapBuilder::with_capacity(1_000)`
- `NewsSitemapBuilder::with_capacity(1_000)`
- `CombinedSitemapBuilder::with_capacity(500)`
- `SitemapIndexBuilder::with_capacity(50)`

### Benchmarks

| Operation              | 10 URLs  | 100 URLs | 1,000 URLs | 10,000 URLs |
|------------------------|----------|----------|------------|-------------|
| Standard Sitemap       | 12.5 µs  | 120 µs   | 1.2 ms     | 12.2 ms     |
| Build Bytes            | 5.7 µs   | 51 µs    | 519 µs     | 5.2 ms      |
| Build Compressed       | 25 µs    | 180 µs   | 1.8 ms     | 18 ms       |
| Image Sitemap (2 imgs) | 28 µs    | 275 µs   | 2.7 ms     | -           |
| Video Sitemap          | 45 µs    | 450 µs   | 4.5 ms     | -           |

**Compression Ratios**: 31x (100 URLs) to 54x (10,000 URLs) - saves 96-98% bandwidth

**Proven Results**:
- Performance: [Criterion.rs](https://github.com/bheisler/criterion.rs) benchmarks
- Memory safety: Custom allocator tracking (see [MEMORY_SAFETY.md](docs/MEMORY_SAFETY.md))

**Documentation**:
- [BENCHMARK_RESULTS.md](BENCHMARK_RESULTS.md) - Actual benchmark numbers
- [BENCHMARKS.md](BENCHMARKS.md) - Performance analysis
- [MEMORY_SAFETY.md](docs/MEMORY_SAFETY.md) - Memory leak proof

Run benchmarks yourself:

```bash
# Performance benchmarks
cargo bench
open target/criterion/report/index.html

# Memory safety demo
cargo run --example memory_demo --release
```

## Validation

The library automatically validates:

- URL format (RFC 3986)
- Maximum 50,000 URLs per standard sitemap (1,000 for news sitemaps)
- Maximum 50MB uncompressed size
- URL length (max 2048 characters)
- Date format (W3C Datetime)
- Priority values (0.0 to 1.0)
- Video duration (max 28,800 seconds)
- Video rating (0.0 to 5.0)
- Video title length (max 100 characters)
- Video description length (max 2048 characters)
- News language codes (ISO 639 format)
- Stock tickers (max 5, comma-separated)

You can disable validation if needed:

```rust
let builder = SitemapBuilder::new().validate(false);

// Or with capacity
let builder = SitemapBuilder::with_capacity(10_000).validate(false);
```

## Sitemap Protocol Compliance

This library follows the [sitemaps.org protocol 0.9](https://www.sitemaps.org/protocol.html) specification:

- Standard sitemap with `<urlset>` and `<url>` elements
- Optional `<lastmod>`, `<changefreq>`, and `<priority>` elements
- Image extension: `xmlns:image="http://www.google.com/schemas/sitemap-image/1.1"`
- Video extension: `xmlns:video="http://www.google.com/schemas/sitemap-video/1.1"`
- News extension: `xmlns:news="http://www.google.com/schemas/sitemap-news/0.9"`
- Sitemap index with `<sitemapindex>` and `<sitemap>` elements

## Examples

See the [examples](examples/) directory for more complete examples:

- [basic_sitemap.rs](examples/basic_sitemap.rs) - Standard sitemap generation
- [image_sitemap.rs](examples/image_sitemap.rs) - Image sitemap with metadata
- [video_sitemap.rs](examples/video_sitemap.rs) - Video sitemap with full metadata
- [news_sitemap.rs](examples/news_sitemap.rs) - News sitemap for Google News
- [combined_sitemap.rs](examples/combined_sitemap.rs) - Combined sitemap with multiple extensions
- [sitemap_index.rs](examples/sitemap_index.rs) - Sitemap index for multiple files
- [web_framework_usage.rs](examples/web_framework_usage.rs) - Web framework integration examples
- [memory_demo.rs](examples/memory_demo.rs) - Memory tracking and cleanup proof

Run examples with:

```bash
cargo run --example basic_sitemap
cargo run --example image_sitemap
cargo run --example video_sitemap
cargo run --example news_sitemap
cargo run --example combined_sitemap
cargo run --example sitemap_index
cargo run --example web_framework_usage
cargo run --example memory_demo --release
```

## Testing

Run the test suite:

```bash
cargo test
```

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- [Sitemaps.org Protocol](https://www.sitemaps.org/protocol.html)
- [Google Image Sitemaps](https://developers.google.com/search/docs/crawling-indexing/sitemaps/image-sitemaps)
- [Google Video Sitemaps](https://developers.google.com/search/docs/crawling-indexing/sitemaps/video-sitemaps)
- [Google News Sitemaps](https://developers.google.com/search/docs/crawling-indexing/sitemaps/news-sitemap)
- [Combining Sitemap Extensions](https://developers.google.com/search/docs/crawling-indexing/sitemaps/combine-sitemap-extensions)
- [Sitemap Index Files](https://www.sitemaps.org/protocol.html#index)
