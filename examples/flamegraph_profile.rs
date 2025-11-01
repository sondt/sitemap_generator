/// Example for flamegraph profiling
///
/// Run with: cargo flamegraph --example flamegraph_profile --release
///
/// This will generate a flamegraph.svg showing CPU and memory usage

use sitemap_generator::{
    SitemapBuilder, ImageSitemapBuilder, VideoSitemapBuilder,
    NewsSitemapBuilder, CombinedSitemapBuilder,
    UrlEntry, UrlWithImages, UrlWithVideos, UrlWithNews, UrlWithExtensions,
    ImageEntry, VideoEntry, NewsEntry, NewsPublication, ChangeFreq
};

fn main() {
    println!("=== Flamegraph Profiling ===\n");

    // Test 1: Standard sitemap with many URLs
    println!("1. Generating standard sitemap (10,000 URLs)...");
    profile_standard_sitemap(10_000);

    // Test 2: Image sitemap
    println!("2. Generating image sitemap (1,000 URLs)...");
    profile_image_sitemap(1_000);

    // Test 3: Video sitemap
    println!("3. Generating video sitemap (1,000 URLs)...");
    profile_video_sitemap(1_000);

    // Test 4: News sitemap
    println!("4. Generating news sitemap (1,000 URLs)...");
    profile_news_sitemap(1_000);

    // Test 5: Combined sitemap
    println!("5. Generating combined sitemap (500 URLs)...");
    profile_combined_sitemap(500);

    // Test 6: Multiple iterations to show memory cleanup
    println!("6. Running 100 iterations to demonstrate memory cleanup...");
    for i in 0..100 {
        if i % 10 == 0 {
            println!("  Iteration {}...", i);
        }
        profile_standard_sitemap(100);
    }

    println!("\n✓ Profiling complete!");
    println!("Check flamegraph.svg for visualization");
}

fn profile_standard_sitemap(count: usize) {
    let mut builder = SitemapBuilder::new();

    for i in 0..count {
        builder.add_url(
            UrlEntry::new(format!("https://example.com/page-{}", i))
                .lastmod("2025-11-01")
                .changefreq(ChangeFreq::Weekly)
                .priority(0.8)
        );
    }

    let _xml = builder.build().unwrap();
    // Builder and xml dropped here - memory should be freed
}

fn profile_image_sitemap(count: usize) {
    let mut builder = ImageSitemapBuilder::new();

    for i in 0..count {
        let url_with_images = UrlWithImages::new(
            UrlEntry::new(format!("https://example.com/gallery-{}", i))
        )
        .add_image(
            ImageEntry::new(format!("https://example.com/image-{}.jpg", i))
                .title(format!("Image {}", i))
        );

        builder.add_url(url_with_images);
    }

    let _xml = builder.build().unwrap();
}

fn profile_video_sitemap(count: usize) {
    let mut builder = VideoSitemapBuilder::new();

    for i in 0..count {
        let url_with_video = UrlWithVideos::new(
            UrlEntry::new(format!("https://example.com/video-{}", i))
        )
        .add_video(
            VideoEntry::new(
                format!("https://example.com/thumb-{}.jpg", i),
                format!("Video {}", i),
                format!("Description for video {}", i)
            )
            .duration(300)
            .rating(4.5)
        );

        builder.add_url(url_with_video);
    }

    let _xml = builder.build().unwrap();
}

fn profile_news_sitemap(count: usize) {
    let mut builder = NewsSitemapBuilder::new();

    for i in 0..count {
        let news = NewsEntry::new(
            NewsPublication::new("News", "en"),
            "2025-11-01",
            format!("Article {}", i)
        );

        builder.add_url(UrlWithNews::new(
            UrlEntry::new(format!("https://example.com/news-{}", i)),
            news
        ));
    }

    let _xml = builder.build().unwrap();
}

fn profile_combined_sitemap(count: usize) {
    let mut builder = CombinedSitemapBuilder::new();

    for i in 0..count {
        let news = NewsEntry::new(
            NewsPublication::new("News", "en"),
            "2025-11-01",
            format!("Article {}", i)
        );

        let image = ImageEntry::new(format!("https://example.com/image-{}.jpg", i));

        let video = VideoEntry::new(
            format!("https://example.com/thumb-{}.jpg", i),
            format!("Video {}", i),
            "Description"
        );

        builder.add_url(
            UrlWithExtensions::new(UrlEntry::new(format!("https://example.com/combined-{}", i)))
                .add_image(image)
                .add_video(video)
                .set_news(news)
        );
    }

    let _xml = builder.build().unwrap();
}
