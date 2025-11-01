/// Example: Creating a combined sitemap with multiple extensions
///
/// This demonstrates how to combine image, video, and news extensions
/// in a single sitemap. This is useful for rich content pages like
/// news articles with embedded images and videos.

use sitemap_generator::{
    CombinedSitemapBuilder, UrlEntry, UrlWithExtensions,
    ImageEntry, VideoEntry, NewsEntry, NewsPublication,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Combined Sitemap Example ===\n");

    let mut builder = CombinedSitemapBuilder::new();

    // Example 1: News article with images and video
    println!("1. News article with images and video");
    let url = UrlEntry::new("https://example.com/articles/tech-breakthrough")
        .lastmod("2025-11-01");

    let news = NewsEntry::new(
        NewsPublication::new("TechDaily", "en"),
        "2025-11-01T10:00:00Z",
        "Revolutionary AI Breakthrough Announced"
    )
    .keywords("AI, technology, innovation")
    .stock_tickers("GOOGL, MSFT");

    let image1 = ImageEntry::new("https://example.com/images/ai-lab.jpg")
        .title("AI Research Lab")
        .caption("Scientists working on AI breakthrough");

    let image2 = ImageEntry::new("https://example.com/images/ai-demo.jpg")
        .title("AI Demo")
        .caption("Live demonstration of the new AI system");

    let video = VideoEntry::new(
        "https://example.com/thumbs/ai-video.jpg",
        "AI Breakthrough Explained",
        "Watch our interview with the lead researcher"
    )
    .content_loc("https://example.com/videos/ai-interview.mp4")
        .duration(300)
        .family_friendly(true);

    builder.add_url(
        UrlWithExtensions::new(url)
            .add_image(image1)
            .add_image(image2)
            .add_video(video)
            .set_news(news)
    );

    // Example 2: Video page with multiple images (no news)
    println!("2. Video tutorial with thumbnail images");
    let url = UrlEntry::new("https://example.com/tutorials/rust-programming");

    let tutorial_video = VideoEntry::new(
        "https://example.com/thumbs/rust-tutorial.jpg",
        "Learn Rust Programming",
        "Complete Rust programming tutorial for beginners"
    )
    .content_loc("https://example.com/videos/rust-tutorial.mp4")
    .duration(1800)
    .add_tag("programming")
    .add_tag("rust")
    .add_tag("tutorial");

    let code_image = ImageEntry::new("https://example.com/images/rust-code.jpg")
        .title("Rust Code Example");

    builder.add_url(
        UrlWithExtensions::new(url)
            .add_video(tutorial_video)
            .add_image(code_image)
    );

    // Example 3: Image gallery (images only)
    println!("3. Image gallery page");
    let url = UrlEntry::new("https://example.com/gallery/nature");

    let mut gallery = UrlWithExtensions::new(url);
    for i in 1..=5 {
        let image = ImageEntry::new(format!("https://example.com/images/nature-{}.jpg", i))
            .title(format!("Nature Photo {}", i));
        gallery = gallery.add_image(image);
    }

    builder.add_url(gallery);

    // Example 4: Vietnamese news with video
    println!("4. Vietnamese news article with video");
    let url = UrlEntry::new("https://example.com/vi/tin-tuc/cong-nghe");

    let vn_news = NewsEntry::new(
        NewsPublication::new("Báo pháp luật Việt nam", "vi"),
        "2025-11-01T14:00:00+07:00",
        "Tự chủ, tự cường & tự tin - Tầm nhìn mới cho kỷ nguyên vươn mình của dân tộc"
    )
    .keywords("kỷ nguyên vươn mình, tự chủ, công nghệ, đổi mới");

    let vn_video = VideoEntry::new(
        "https://example.com/thumbs/vn-tech.jpg",
        "Giới thiệu công nghệ mới",
        "Video giới thiệu về công nghệ AI mới nhất"
    )
    .content_loc("https://example.com/videos/vn-tech.mp4")
    .duration(240);

    builder.add_url(
        UrlWithExtensions::new(url)
            .set_news(vn_news)
            .add_video(vn_video)
    );

    println!("\nTotal URLs: {}", builder.len());

    // Generate XML
    println!("\n--- Generating Combined Sitemap ---");
    let xml = builder.build()?;
    println!("XML length: {} bytes", xml.len());

    // Show sample XML
    println!("\nFirst 800 characters:");
    println!("{}", &xml[..800.min(xml.len())]);

    // Write to file
    println!("\n--- Writing to file ---");
    builder.write("combined_sitemap.xml")?;
    println!("✓ Written to combined_sitemap.xml");

    // Write compressed
    builder.write_compressed("combined_sitemap.xml.gz")?;
    println!("✓ Written compressed to combined_sitemap.xml.gz");

    // Show compression stats
    let compressed = builder.build_compressed_bytes()?;
    let ratio = xml.len() as f64 / compressed.len() as f64;
    println!("\nCompression ratio: {:.1}x", ratio);
    println!("Original size: {} bytes", xml.len());
    println!("Compressed size: {} bytes", compressed.len());
    println!("Bandwidth saved: {:.1}%", (1.0 - 1.0 / ratio) * 100.0);

    // Best practices
    println!("\n=== Best Practices ===");
    println!("1. Combine extensions when content naturally fits multiple types");
    println!("2. If any URL has news metadata, max 1,000 URLs per sitemap");
    println!("3. Without news: max 50,000 URLs per sitemap");
    println!("4. Watch file size - combined sitemaps can grow large");
    println!("5. Use compression to save bandwidth (recommended)");
    println!("6. Test with Google Search Console");

    println!("\n✓ Combined sitemap example completed!");

    Ok(())
}
