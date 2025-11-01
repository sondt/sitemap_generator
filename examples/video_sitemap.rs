use sitemap_generator::{
    VideoSitemapBuilder, UrlEntry, UrlWithVideos, VideoEntry,
    VideoUploader, VideoRequiresSubscription,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a video sitemap builder
    let mut builder = VideoSitemapBuilder::new();

    // Add a URL with a video
    let url_with_video = UrlWithVideos::new(
        UrlEntry::new("https://example.com/videos/introduction")
            .lastmod("2025-11-01")
            .priority(0.9)
    )
    .add_video(
        VideoEntry::new(
            "https://example.com/thumbnails/intro.jpg",
            "Introduction to Our Product",
            "Learn everything you need to know about our amazing product in this comprehensive introduction video."
        )
        .content_loc("https://example.com/videos/intro.mp4")
        .player_loc("https://example.com/player?video=intro")
        .duration(300) // 5 minutes
        .publication_date("2025-11-01")
        .rating(4.5)
        .view_count(10000)
        .family_friendly(true)
        .add_tag("tutorial")
        .add_tag("product")
        .add_tag("introduction")
        .category("Education")
        .uploader(
            VideoUploader::new("Example Corp")
                .info_url("https://example.com/about")
        )
        .requires_subscription(VideoRequiresSubscription::No)
    );

    builder.add_url(url_with_video);

    // Add another URL with a different video
    let url_with_video2 = UrlWithVideos::new(
        UrlEntry::new("https://example.com/videos/advanced-features")
            .lastmod("2025-11-15")
            .priority(0.8)
    )
    .add_video(
        VideoEntry::new(
            "https://example.com/thumbnails/advanced.jpg",
            "Advanced Features Deep Dive",
            "Explore the advanced features that make our product stand out from the competition."
        )
        .content_loc("https://example.com/videos/advanced.mp4")
        .duration(600) // 10 minutes
        .publication_date("2025-11-15")
        .rating(4.8)
        .view_count(5000)
        .family_friendly(true)
        .add_tag("advanced")
        .add_tag("features")
        .category("Education")
        .requires_subscription(VideoRequiresSubscription::Yes)
    );

    builder.add_url(url_with_video2);

    // Generate the XML
    let xml = builder.build()?;
    println!("Generated video sitemap ({} bytes):", xml.len());
    println!("{}", xml);

    // Write to file
    builder.write("video_sitemap.xml")?;
    println!("\nVideo sitemap written to video_sitemap.xml");

    Ok(())
}
