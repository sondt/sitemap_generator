use sitemap_generator::*;
use tempfile::NamedTempFile;

#[test]
fn test_basic_sitemap_generation() {
    let mut builder = SitemapBuilder::new();

    builder.add_url(
        UrlEntry::new("https://example.com/")
            .lastmod("2025-11-01")
            .changefreq(ChangeFreq::Daily)
            .priority(1.0),
    );

    builder.add_url(
        UrlEntry::new("https://example.com/about")
            .lastmod("2025-11-02")
            .priority(0.8),
    );

    let xml = builder.build().unwrap();

    // Verify XML contains expected elements
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"));
    assert!(xml.contains("<loc>https://example.com/</loc>"));
    assert!(xml.contains("<lastmod>2025-11-01</lastmod>"));
    assert!(xml.contains("<changefreq>daily</changefreq>"));
    assert!(xml.contains("<priority>1</priority>"));
}

#[test]
fn test_image_sitemap_generation() {
    let mut builder = ImageSitemapBuilder::new();

    let url_with_images = UrlWithImages::new(
        UrlEntry::new("https://example.com/gallery")
            .lastmod("2025-11-01")
            .priority(0.9),
    )
    .add_image(
        ImageEntry::new("https://example.com/images/photo1.jpg")
            .title("Beautiful Photo")
            .caption("A stunning landscape"),
    );

    builder.add_url(url_with_images);

    let xml = builder.build().unwrap();

    // Verify XML contains image namespace and elements
    assert!(xml.contains("xmlns:image=\"http://www.google.com/schemas/sitemap-image/1.1\""));
    assert!(xml.contains("<image:image>"));
    assert!(xml.contains("<image:loc>https://example.com/images/photo1.jpg</image:loc>"));
    assert!(xml.contains("<image:title>Beautiful Photo</image:title>"));
    assert!(xml.contains("<image:caption>A stunning landscape</image:caption>"));
}

#[test]
fn test_video_sitemap_generation() {
    let mut builder = VideoSitemapBuilder::new();

    let url_with_video = UrlWithVideos::new(UrlEntry::new("https://example.com/videos/intro"))
        .add_video(
            VideoEntry::new(
                "https://example.com/thumbnails/intro.jpg",
                "Introduction Video",
                "Learn about our product",
            )
            .content_loc("https://example.com/videos/intro.mp4")
            .duration(300)
            .rating(4.5)
            .family_friendly(true),
        );

    builder.add_url(url_with_video);

    let xml = builder.build().unwrap();

    // Verify XML contains video namespace and elements
    assert!(xml.contains("xmlns:video=\"http://www.google.com/schemas/sitemap-video/1.1\""));
    assert!(xml.contains("<video:video>"));
    assert!(xml.contains("<video:thumbnail_loc>https://example.com/thumbnails/intro.jpg</video:thumbnail_loc>"));
    assert!(xml.contains("<video:title>Introduction Video</video:title>"));
    assert!(xml.contains("<video:duration>300</video:duration>"));
    assert!(xml.contains("<video:rating>4.5</video:rating>"));
    assert!(xml.contains("<video:family_friendly>yes</video:family_friendly>"));
}

#[test]
fn test_sitemap_index_generation() {
    let mut builder = SitemapIndexBuilder::new();

    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/sitemap1.xml.gz").lastmod("2025-11-01"),
    );

    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/sitemap2.xml.gz").lastmod("2025-11-02"),
    );

    let xml = builder.build().unwrap();

    // Verify XML contains sitemapindex elements
    assert!(xml.contains("<sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"));
    assert!(xml.contains("<sitemap>"));
    assert!(xml.contains("<loc>https://example.com/sitemap1.xml.gz</loc>"));
    assert!(xml.contains("<loc>https://example.com/sitemap2.xml.gz</loc>"));
}

#[test]
fn test_file_writing() {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    builder.write(path).unwrap();

    let content = std::fs::read_to_string(path).unwrap();
    assert!(content.contains("<loc>https://example.com/</loc>"));
}

#[test]
fn test_compressed_file_writing() {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    builder.write_compressed(path).unwrap();

    // Verify file exists and has content
    let metadata = std::fs::metadata(path).unwrap();
    assert!(metadata.len() > 0);
}

#[test]
fn test_validation_url_limit() {
    let mut builder = SitemapBuilder::new();

    // Add 50,001 URLs (over the limit)
    for i in 0..50_001 {
        builder.add_url(UrlEntry::new(format!("https://example.com/page{}", i)));
    }

    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_validation_invalid_url() {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("not a valid url"));

    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_validation_invalid_priority() {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/").priority(1.5));

    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_validation_disabled() {
    let mut builder = SitemapBuilder::new().validate(false);
    builder.add_url(UrlEntry::new("not a valid url"));

    // Should succeed when validation is disabled
    let result = builder.build();
    assert!(result.is_ok());
}

#[test]
fn test_parser_roundtrip() {
    let mut builder = SitemapBuilder::new();

    builder.add_url(
        UrlEntry::new("https://example.com/")
            .lastmod("2025-11-01")
            .changefreq(ChangeFreq::Daily)
            .priority(1.0),
    );

    builder.add_url(
        UrlEntry::new("https://example.com/about")
            .lastmod("2025-11-02")
            .priority(0.8),
    );

    let xml = builder.build().unwrap();

    // Parse the generated XML
    let entries = SitemapParser::parse_string(&xml).unwrap();

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].loc, "https://example.com/");
    assert_eq!(entries[0].lastmod, Some("2025-11-01".to_string()));
    assert_eq!(entries[0].changefreq, Some(ChangeFreq::Daily));
    assert_eq!(entries[0].priority, Some(1.0));

    assert_eq!(entries[1].loc, "https://example.com/about");
    assert_eq!(entries[1].lastmod, Some("2025-11-02".to_string()));
    assert_eq!(entries[1].priority, Some(0.8));
}

#[test]
fn test_xml_escaping() {
    let mut builder = SitemapBuilder::new();

    // Add a URL with special characters that need escaping
    builder.add_url(UrlEntry::new("https://example.com/page?foo=bar&baz=qux"));

    let xml = builder.build().unwrap();

    // Verify special characters are escaped
    assert!(xml.contains("&amp;"));
}

#[test]
fn test_empty_sitemap() {
    let builder = SitemapBuilder::new();

    let xml = builder.build().unwrap();

    // Should generate valid empty sitemap
    assert!(xml.contains("<urlset"));
    assert!(xml.contains("</urlset>"));
}

#[test]
fn test_video_with_all_fields() {
    let mut builder = VideoSitemapBuilder::new();

    let url_with_video = UrlWithVideos::new(UrlEntry::new("https://example.com/videos/full"))
        .add_video(
            VideoEntry::new(
                "https://example.com/thumbnails/full.jpg",
                "Full Video",
                "Video with all fields",
            )
            .content_loc("https://example.com/videos/full.mp4")
            .player_loc("https://example.com/player?video=full")
            .duration(600)
            .publication_date("2025-11-01")
            .expiration_date("2025-01-01")
            .rating(4.5)
            .view_count(10000)
            .family_friendly(true)
            .add_tag("tag1")
            .add_tag("tag2")
            .category("Education")
            .requires_subscription(VideoRequiresSubscription::Yes)
            .uploader(VideoUploader::new("Test User").info_url("https://example.com/user")),
        );

    builder.add_url(url_with_video);

    let xml = builder.build().unwrap();

    // Verify all fields are present
    assert!(xml.contains("<video:content_loc>"));
    assert!(xml.contains("<video:player_loc>"));
    assert!(xml.contains("<video:duration>600</video:duration>"));
    assert!(xml.contains("<video:publication_date>2025-11-01</video:publication_date>"));
    assert!(xml.contains("<video:expiration_date>2025-01-01</video:expiration_date>"));
    assert!(xml.contains("<video:rating>4.5</video:rating>"));
    assert!(xml.contains("<video:view_count>10000</video:view_count>"));
    assert!(xml.contains("<video:tag>tag1</video:tag>"));
    assert!(xml.contains("<video:tag>tag2</video:tag>"));
    assert!(xml.contains("<video:category>Education</video:category>"));
    assert!(xml.contains("<video:requires_subscription>yes</video:requires_subscription>"));
    assert!(xml.contains("<video:uploader info=\"https://example.com/user\">Test User</video:uploader>"));
}

#[test]
fn test_build_bytes() {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    let bytes = builder.build_bytes().unwrap();

    // Verify it's valid UTF-8 XML
    let xml_string = String::from_utf8(bytes.clone()).unwrap();
    assert!(xml_string.contains("<loc>https://example.com/</loc>"));

    // Verify bytes are not empty
    assert!(!bytes.is_empty());
}

#[test]
fn test_build_compressed_bytes() {
    let mut builder = SitemapBuilder::new();
    builder.add_url(UrlEntry::new("https://example.com/"));

    let bytes = builder.build_bytes().unwrap();
    let compressed = builder.build_compressed_bytes().unwrap();

    // Compressed should be smaller than original
    assert!(compressed.len() < bytes.len());

    // Compressed should not be empty
    assert!(!compressed.is_empty());
}

#[test]
fn test_image_sitemap_bytes() {
    let mut builder = ImageSitemapBuilder::new();

    let url_with_images = UrlWithImages::new(
        UrlEntry::new("https://example.com/gallery")
    )
    .add_image(
        ImageEntry::new("https://example.com/images/photo1.jpg")
            .title("Test Photo")
    );

    builder.add_url(url_with_images);

    let bytes = builder.build_bytes().unwrap();
    let compressed = builder.build_compressed_bytes().unwrap();

    assert!(!bytes.is_empty());
    assert!(!compressed.is_empty());
    assert!(compressed.len() < bytes.len());
}

#[test]
fn test_video_sitemap_bytes() {
    let mut builder = VideoSitemapBuilder::new();

    let url_with_video = UrlWithVideos::new(
        UrlEntry::new("https://example.com/videos/test")
    )
    .add_video(
        VideoEntry::new(
            "https://example.com/thumbnails/test.jpg",
            "Test Video",
            "A test video"
        )
    );

    builder.add_url(url_with_video);

    let bytes = builder.build_bytes().unwrap();
    let compressed = builder.build_compressed_bytes().unwrap();

    assert!(!bytes.is_empty());
    assert!(!compressed.is_empty());
    assert!(compressed.len() < bytes.len());
}

#[test]
fn test_sitemap_index_bytes() {
    let mut builder = SitemapIndexBuilder::new();
    builder.add_sitemap(
        SitemapIndexEntry::new("https://example.com/sitemap1.xml.gz")
    );

    let bytes = builder.build_bytes().unwrap();
    let compressed = builder.build_compressed_bytes().unwrap();

    assert!(!bytes.is_empty());
    assert!(!compressed.is_empty());
    assert!(compressed.len() < bytes.len());
}

#[test]
fn test_compression_ratio() {
    let mut builder = SitemapBuilder::new();

    // Add many URLs to test compression effectiveness
    for i in 0..100 {
        builder.add_url(UrlEntry::new(format!("https://example.com/page{}", i)));
    }

    let bytes = builder.build_bytes().unwrap();
    let compressed = builder.build_compressed_bytes().unwrap();

    let ratio = bytes.len() as f64 / compressed.len() as f64;

    // Compression should be at least 2x for repetitive XML
    assert!(ratio > 2.0, "Compression ratio should be > 2x, got {:.2}x", ratio);
}

// News sitemap tests

#[test]
fn test_news_sitemap_basic() {
    use sitemap_generator::{NewsSitemapBuilder, NewsEntry, NewsPublication, UrlEntry, UrlWithNews};

    let mut builder = NewsSitemapBuilder::new();

    let publication = NewsPublication::new("TechNews", "en");
    let news = NewsEntry::new(
        publication,
        "2025-11-01T10:00:00Z",
        "Breaking Tech News"
    );
    let url = UrlEntry::new("https://example.com/news/tech");

    builder.add_url(UrlWithNews::new(url, news));

    let xml = builder.build().unwrap();
    assert!(xml.contains("xmlns:news"));
    assert!(xml.contains("TechNews"));
    assert!(xml.contains("Breaking Tech News"));
    assert!(xml.contains("2025-11-01T10:00:00Z"));
}

#[test]
fn test_news_sitemap_with_keywords() {
    use sitemap_generator::{NewsSitemapBuilder, NewsEntry, NewsPublication, UrlEntry, UrlWithNews};

    let mut builder = NewsSitemapBuilder::new();

    let publication = NewsPublication::new("BusinessNews", "en");
    let news = NewsEntry::new(
        publication,
        "2025-11-01T09:00:00Z",
        "Market Update"
    )
    .keywords("business, finance, market")
    .stock_tickers("AAPL, GOOGL");

    let url = UrlEntry::new("https://example.com/business/market");
    builder.add_url(UrlWithNews::new(url, news));

    let xml = builder.build().unwrap();
    assert!(xml.contains("business, finance, market"));
    assert!(xml.contains("AAPL, GOOGL"));
}

#[test]
fn test_news_sitemap_multiple_languages() {
    use sitemap_generator::{NewsSitemapBuilder, NewsEntry, NewsPublication, UrlEntry, UrlWithNews};

    let mut builder = NewsSitemapBuilder::new();

    // English article
    let pub_en = NewsPublication::new("News", "en");
    let news_en = NewsEntry::new(pub_en, "2025-11-01", "English Article");
    builder.add_url(UrlWithNews::new(
        UrlEntry::new("https://example.com/en/article"),
        news_en
    ));

    // Vietnamese article
    let pub_vi = NewsPublication::new("Tin tức", "vi");
    let news_vi = NewsEntry::new(pub_vi, "2025-11-01", "Bài viết tiếng Việt");
    builder.add_url(UrlWithNews::new(
        UrlEntry::new("https://example.com/vi/bai-viet"),
        news_vi
    ));

    // Chinese article
    let pub_zh = NewsPublication::new("新闻", "zh-cn");
    let news_zh = NewsEntry::new(pub_zh, "2025-11-01", "中文文章");
    builder.add_url(UrlWithNews::new(
        UrlEntry::new("https://example.com/zh/article"),
        news_zh
    ));

    let xml = builder.build().unwrap();
    assert!(xml.contains("<news:language>en</news:language>"));
    assert!(xml.contains("<news:language>vi</news:language>"));
    assert!(xml.contains("<news:language>zh-cn</news:language>"));
}

#[test]
fn test_news_sitemap_validation() {
    use sitemap_generator::{NewsSitemapBuilder, NewsEntry, NewsPublication, UrlEntry, UrlWithNews};

    let mut builder = NewsSitemapBuilder::new();

    // Add 1,000 articles (max allowed)
    for i in 0..1000 {
        let publication = NewsPublication::new("News", "en");
        let news = NewsEntry::new(
            publication,
            "2025-11-01",
            format!("Article {}", i)
        );
        builder.add_url(UrlWithNews::new(
            UrlEntry::new(format!("https://example.com/article-{}", i)),
            news
        ));
    }

    // Should succeed with 1,000 URLs
    assert!(builder.build().is_ok());

    // Add one more to exceed limit
    let publication = NewsPublication::new("News", "en");
    let news = NewsEntry::new(publication, "2025-11-01", "Extra Article");
    builder.add_url(UrlWithNews::new(
        UrlEntry::new("https://example.com/article-1000"),
        news
    ));

    // Should fail with 1,001 URLs
    assert!(builder.build().is_err());
}

#[test]
fn test_news_sitemap_invalid_language() {
    use sitemap_generator::{NewsSitemapBuilder, NewsEntry, NewsPublication, UrlEntry, UrlWithNews};

    let mut builder = NewsSitemapBuilder::new();

    // Invalid language code (uppercase)
    let publication = NewsPublication::new("News", "EN");
    let news = NewsEntry::new(publication, "2025-11-01", "Article");
    builder.add_url(UrlWithNews::new(
        UrlEntry::new("https://example.com/article"),
        news
    ));

    // Should fail validation
    assert!(builder.build().is_err());
}

#[test]
fn test_news_sitemap_bytes() {
    use sitemap_generator::{NewsSitemapBuilder, NewsEntry, NewsPublication, UrlEntry, UrlWithNews};

    let mut builder = NewsSitemapBuilder::new();

    let publication = NewsPublication::new("News", "en");
    let news = NewsEntry::new(publication, "2025-11-01", "Test Article");
    builder.add_url(UrlWithNews::new(
        UrlEntry::new("https://example.com/test"),
        news
    ));

    let bytes = builder.build_bytes().unwrap();
    let compressed = builder.build_compressed_bytes().unwrap();

    assert!(!bytes.is_empty());
    assert!(!compressed.is_empty());
    assert!(compressed.len() < bytes.len());
}

#[test]
fn test_news_sitemap_file_write() {
    use sitemap_generator::{NewsSitemapBuilder, NewsEntry, NewsPublication, UrlEntry, UrlWithNews};
    use std::fs;

    let mut builder = NewsSitemapBuilder::new();

    let publication = NewsPublication::new("News", "en");
    let news = NewsEntry::new(publication, "2025-11-01", "Test Article");
    builder.add_url(UrlWithNews::new(
        UrlEntry::new("https://example.com/test"),
        news
    ));

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("news_sitemap.xml");
    let compressed_path = temp_dir.path().join("news_sitemap.xml.gz");

    // Write files
    builder.write(&file_path).unwrap();
    builder.write_compressed(&compressed_path).unwrap();

    // Verify files exist and have content
    assert!(file_path.exists());
    assert!(compressed_path.exists());

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("xmlns:news"));
    assert!(content.contains("Test Article"));
}

// Combined sitemap tests

#[test]
fn test_combined_sitemap_all_extensions() {
    use sitemap_generator::{
        CombinedSitemapBuilder, UrlEntry, UrlWithExtensions,
        ImageEntry, VideoEntry, NewsEntry, NewsPublication
    };

    let mut builder = CombinedSitemapBuilder::new();

    let news = NewsEntry::new(
        NewsPublication::new("News", "en"),
        "2025-11-01",
        "Test Article"
    );

    let image = ImageEntry::new("https://example.com/image.jpg");
    let video = VideoEntry::new(
        "https://example.com/thumb.jpg",
        "Test Video",
        "Test Description"
    );

    builder.add_url(
        UrlWithExtensions::new(UrlEntry::new("https://example.com/article"))
            .add_image(image)
            .add_video(video)
            .set_news(news)
    );

    let xml = builder.build().unwrap();
    assert!(xml.contains("xmlns:image"));
    assert!(xml.contains("xmlns:video"));
    assert!(xml.contains("xmlns:news"));
    assert!(xml.contains("image.jpg"));
    assert!(xml.contains("Test Video"));
    assert!(xml.contains("Test Article"));
}

#[test]
fn test_combined_sitemap_partial_extensions() {
    use sitemap_generator::{
        CombinedSitemapBuilder, UrlEntry, UrlWithExtensions, ImageEntry
    };

    let mut builder = CombinedSitemapBuilder::new();

    // URL with only images (no video, no news)
    builder.add_url(
        UrlWithExtensions::new(UrlEntry::new("https://example.com/gallery"))
            .add_image(ImageEntry::new("https://example.com/photo1.jpg"))
            .add_image(ImageEntry::new("https://example.com/photo2.jpg"))
    );

    let xml = builder.build().unwrap();
    assert!(xml.contains("xmlns:image"));
    assert!(!xml.contains("xmlns:video"));
    assert!(!xml.contains("xmlns:news"));
    assert!(xml.contains("photo1.jpg"));
    assert!(xml.contains("photo2.jpg"));
}

#[test]
fn test_combined_sitemap_news_url_limit() {
    use sitemap_generator::{
        CombinedSitemapBuilder, UrlEntry, UrlWithExtensions,
        NewsEntry, NewsPublication
    };

    let mut builder = CombinedSitemapBuilder::new();

    // Add 1,000 URLs with news (max allowed for news sitemaps)
    for i in 0..1000 {
        let news = NewsEntry::new(
            NewsPublication::new("News", "en"),
            "2025-11-01",
            format!("Article {}", i)
        );

        builder.add_url(
            UrlWithExtensions::new(UrlEntry::new(format!("https://example.com/{}", i)))
                .set_news(news)
        );
    }

    // Should succeed with 1,000 URLs
    assert!(builder.build().is_ok());

    // Add one more to exceed limit
    let news = NewsEntry::new(
        NewsPublication::new("News", "en"),
        "2025-11-01",
        "Extra"
    );
    builder.add_url(
        UrlWithExtensions::new(UrlEntry::new("https://example.com/1000"))
            .set_news(news)
    );

    // Should fail with 1,001 URLs containing news
    assert!(builder.build().is_err());
}

#[test]
fn test_combined_sitemap_without_news_url_limit() {
    use sitemap_generator::{
        CombinedSitemapBuilder, UrlEntry, UrlWithExtensions, ImageEntry
    };

    let mut builder = CombinedSitemapBuilder::new();

    // Add 1,001 URLs without news (should be OK, limit is 50,000 for non-news)
    for i in 0..1001 {
        builder.add_url(
            UrlWithExtensions::new(UrlEntry::new(format!("https://example.com/{}", i)))
                .add_image(ImageEntry::new("https://example.com/image.jpg"))
        );
    }

    // Should succeed since there's no news metadata
    assert!(builder.build().is_ok());
}

#[test]
fn test_combined_sitemap_bytes() {
    use sitemap_generator::{
        CombinedSitemapBuilder, UrlEntry, UrlWithExtensions, ImageEntry
    };

    let mut builder = CombinedSitemapBuilder::new();

    builder.add_url(
        UrlWithExtensions::new(UrlEntry::new("https://example.com/test"))
            .add_image(ImageEntry::new("https://example.com/test.jpg"))
    );

    let bytes = builder.build_bytes().unwrap();
    let compressed = builder.build_compressed_bytes().unwrap();

    assert!(!bytes.is_empty());
    assert!(!compressed.is_empty());
    assert!(compressed.len() < bytes.len());
}

#[test]
fn test_combined_sitemap_file_write() {
    use sitemap_generator::{
        CombinedSitemapBuilder, UrlEntry, UrlWithExtensions,
        ImageEntry, VideoEntry
    };
    use std::fs;

    let mut builder = CombinedSitemapBuilder::new();

    let image = ImageEntry::new("https://example.com/image.jpg");
    let video = VideoEntry::new(
        "https://example.com/thumb.jpg",
        "Test",
        "Description"
    );

    builder.add_url(
        UrlWithExtensions::new(UrlEntry::new("https://example.com/test"))
            .add_image(image)
            .add_video(video)
    );

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("combined.xml");
    let compressed_path = temp_dir.path().join("combined.xml.gz");

    builder.write(&file_path).unwrap();
    builder.write_compressed(&compressed_path).unwrap();

    assert!(file_path.exists());
    assert!(compressed_path.exists());

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("xmlns:image"));
    assert!(content.contains("xmlns:video"));
}

// ============================================================================
// with_capacity() Tests
// ============================================================================

#[test]
fn test_sitemap_builder_with_capacity() {
    let mut builder = SitemapBuilder::with_capacity(100);

    for i in 0..100 {
        builder.add_url(UrlEntry::new(format!("https://example.com/page{}", i)));
    }

    assert_eq!(builder.len(), 100);
    let xml = builder.build().unwrap();
    assert!(xml.contains("<loc>https://example.com/page0</loc>"));
    assert!(xml.contains("<loc>https://example.com/page99</loc>"));
}

#[test]
fn test_image_sitemap_builder_with_capacity() {
    let mut builder = ImageSitemapBuilder::with_capacity(50);

    for i in 0..50 {
        let url = UrlWithImages::new(
            UrlEntry::new(format!("https://example.com/gallery{}", i))
        ).add_image(
            ImageEntry::new(format!("https://example.com/img{}.jpg", i))
                .title(format!("Image {}", i))
        );
        builder.add_url(url);
    }

    assert_eq!(builder.len(), 50);
    let xml = builder.build().unwrap();
    assert!(xml.contains("xmlns:image"));
    assert!(xml.contains("<image:loc>https://example.com/img0.jpg</image:loc>"));
}

#[test]
fn test_video_sitemap_builder_with_capacity() {
    let mut builder = VideoSitemapBuilder::with_capacity(30);

    for i in 0..30 {
        let url = UrlWithVideos::new(
            UrlEntry::new(format!("https://example.com/video{}", i))
        ).add_video(
            VideoEntry::new(
                format!("https://example.com/thumb{}.jpg", i),
                format!("Video {}", i),
                "Description"
            ).duration(120)
        );
        builder.add_url(url);
    }

    assert_eq!(builder.len(), 30);
    let xml = builder.build().unwrap();
    assert!(xml.contains("xmlns:video"));
    assert!(xml.contains("<video:title>Video 0</video:title>"));
}

#[test]
fn test_sitemap_index_builder_with_capacity() {
    let mut builder = SitemapIndexBuilder::with_capacity(10);

    for i in 0..10 {
        builder.add_sitemap(
            SitemapIndexEntry::new(format!("https://example.com/sitemap{}.xml.gz", i))
                .lastmod("2025-11-01")
        );
    }

    assert_eq!(builder.len(), 10);
    let xml = builder.build().unwrap();
    assert!(xml.contains("<loc>https://example.com/sitemap0.xml.gz</loc>"));
    assert!(xml.contains("<loc>https://example.com/sitemap9.xml.gz</loc>"));
}

#[test]
fn test_news_sitemap_builder_with_capacity() {
    let mut builder = NewsSitemapBuilder::with_capacity(100);

    for i in 0..100 {
        let publication = NewsPublication::new("TechDaily", "en");
        let news = NewsEntry::new(
            publication,
            "2025-11-01T10:00:00Z",
            format!("Article {}", i)
        );
        builder.add_url(UrlWithNews::new(
            UrlEntry::new(format!("https://example.com/news{}", i)),
            news
        ));
    }

    assert_eq!(builder.len(), 100);
    let xml = builder.build().unwrap();
    assert!(xml.contains("xmlns:news"));
    assert!(xml.contains("<news:title>Article 0</news:title>"));
}

#[test]
fn test_combined_sitemap_builder_with_capacity() {
    let mut builder = CombinedSitemapBuilder::with_capacity(50);

    for i in 0..50 {
        let url = UrlWithExtensions::new(
            UrlEntry::new(format!("https://example.com/page{}", i))
        ).add_image(
            ImageEntry::new(format!("https://example.com/img{}.jpg", i))
        );
        builder.add_url(url);
    }

    assert_eq!(builder.len(), 50);
    let xml = builder.build().unwrap();
    assert!(xml.contains("xmlns:image"));
}

#[test]
fn test_with_capacity_zero() {
    // Should not panic with zero capacity
    let mut builder = SitemapBuilder::with_capacity(0);
    builder.add_url(UrlEntry::new("https://example.com/"));
    assert_eq!(builder.len(), 1);
    assert!(builder.build().is_ok());
}

#[test]
fn test_with_capacity_large() {
    // Test with large capacity (should pre-allocate)
    let mut builder = SitemapBuilder::with_capacity(10_000);

    for i in 0..10_000 {
        builder.add_url(UrlEntry::new(format!("https://example.com/page{}", i)));
    }

    assert_eq!(builder.len(), 10_000);
    assert!(builder.build().is_ok());
}
