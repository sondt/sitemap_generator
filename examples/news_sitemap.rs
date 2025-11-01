/// Example: Creating a Google News sitemap
///
/// This demonstrates how to create a news sitemap for Google News.
/// News sitemaps should only include articles published within the last 2 days.

use sitemap_generator::{NewsSitemapBuilder, UrlEntry, UrlWithNews, NewsEntry, NewsPublication};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== News Sitemap Example ===\n");

    let mut builder = NewsSitemapBuilder::new();

    // Example 1: Tech news article
    let publication = NewsPublication::new("TechDaily", "en");
    let news = NewsEntry::new(
        publication,
        "2025-11-01T10:00:00Z",
        "Revolutionary AI Breakthrough Announced"
    )
    .keywords("AI, technology, machine learning, innovation")
    .stock_tickers("GOOGL, MSFT");

    let url = UrlEntry::new("https://example.com/tech/ai-breakthrough");
    builder.add_url(UrlWithNews::new(url, news));

    // Example 2: Business news article
    let publication = NewsPublication::new("BusinessInsider", "en");
    let news = NewsEntry::new(
        publication,
        "2025-11-01T09:30:00Z",
        "Stock Market Reaches New Heights"
    )
    .keywords("business, finance, stock market, economy")
    .stock_tickers("AAPL, TSLA, AMZN");

    let url = UrlEntry::new("https://example.com/business/stock-market-surge");
    builder.add_url(UrlWithNews::new(url, news));

    // Example 3: Vietnamese news article
    let publication = NewsPublication::new("Báo Pháp luật Việt Nam", "vi");
    let news = NewsEntry::new(
        publication,
        "2025-11-01T08:00:00+07:00",
        "Tự chủ, tự cường & tự tin - Tầm nhìn mới cho kỷ nguyên vươn mình của dân tộc"
    )
    .keywords("kỷ nguyên vươn mình, tự chủ, công nghệ, đổi mới");

    let url = UrlEntry::new("https://example.com/vi/cong-nghe-moi");
    builder.add_url(UrlWithNews::new(url, news));

    // Example 4: Chinese (Simplified) news article
    let publication = NewsPublication::new("人民日报", "zh-cn");
    let news = NewsEntry::new(
        publication,
        "2025-11-01T14:00:00+08:00",
        "科技创新推动经济发展"
    )
    .keywords("科技, 创新, 经济");

    let url = UrlEntry::new("https://example.com/zh/tech-innovation");
    builder.add_url(UrlWithNews::new(url, news));

    // Example 5: Sports news without stock tickers
    let publication = NewsPublication::new("SportNews", "en");
    let news = NewsEntry::new(
        publication,
        "2025-11-01T12:00:00Z",
        "Championship Finals Set to Begin"
    )
    .keywords("sports, championship, finals, competition");

    let url = UrlEntry::new("https://example.com/sports/championship-finals");
    builder.add_url(UrlWithNews::new(url, news));

    println!("Number of articles: {}", builder.len());

    // 1. Generate as String
    println!("\n--- Generating XML String ---");
    let xml = builder.build()?;
    println!("XML length: {} bytes", xml.len());
    println!("\nFirst 500 characters:");
    println!("{}", &xml[..500.min(xml.len())]);

    // 2. Write to file
    println!("\n--- Writing to file ---");
    builder.write("news_sitemap.xml")?;
    println!("✓ Written to news_sitemap.xml");

    // 3. Write compressed (recommended for production)
    builder.write_compressed("news_sitemap.xml.gz")?;
    println!("✓ Written compressed to news_sitemap.xml.gz");

    // Show compression ratio
    let compressed = builder.build_compressed_bytes()?;
    let ratio = xml.len() as f64 / compressed.len() as f64;
    println!("\nCompression ratio: {:.1}x", ratio);
    println!("Original size: {} bytes", xml.len());
    println!("Compressed size: {} bytes", compressed.len());
    println!("Bandwidth saved: {:.1}%", (1.0 - 1.0 / ratio) * 100.0);

    // Best practices reminder
    println!("\n=== Best Practices ===");
    println!("1. Include only articles published within the last 2 days");
    println!("2. Maximum 1,000 URLs per news sitemap");
    println!("3. Update continuously (don't create new sitemaps each time)");
    println!("4. Use compression to save bandwidth");
    println!("5. Submit sitemap to Google Search Console");

    println!("\n✓ News sitemap example completed!");

    Ok(())
}
