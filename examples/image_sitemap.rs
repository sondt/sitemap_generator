use sitemap_generator::{ImageSitemapBuilder, UrlEntry, UrlWithImages, ImageEntry, GeoLocation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an image sitemap builder
    let mut builder = ImageSitemapBuilder::new();

    // Add a URL with images
    let url_with_images = UrlWithImages::new(
        UrlEntry::new("https://example.com/photo-gallery")
            .lastmod("2025-11-01")
            .priority(0.9)
    )
    .add_image(
        ImageEntry::new("https://example.com/images/photo1.jpg")
            .title("Beautiful Sunset")
            .caption("A stunning sunset over the ocean")
            .geo_location(
                GeoLocation::new()
                    .city("San Francisco")
                    .state("CA")
                    .country("USA")
            )
    )
    .add_image(
        ImageEntry::new("https://example.com/images/photo2.jpg")
            .title("Mountain View")
            .caption("Snow-capped mountains in winter")
            .license("https://creativecommons.org/licenses/by/4.0/")
    );

    builder.add_url(url_with_images);

    // Add another URL with a single image
    let url_with_image = UrlWithImages::new(
        UrlEntry::new("https://example.com/product/widget-pro")
            .lastmod("2025-11-05")
            .priority(0.8)
    )
    .add_image(
        ImageEntry::new("https://example.com/images/widget-pro.jpg")
            .title("Widget Pro")
            .caption("The ultimate widget for all your needs")
    );

    builder.add_url(url_with_image);

    // Generate the XML
    let xml = builder.build()?;
    println!("Generated image sitemap ({} bytes):", xml.len());
    println!("{}", xml);

    // Write to file
    builder.write("image_sitemap.xml")?;
    println!("\nImage sitemap written to image_sitemap.xml");

    Ok(())
}
