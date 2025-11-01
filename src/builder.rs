//! Builder patterns for creating sitemaps

use crate::error::Result;
use crate::types::*;
use crate::validator::Validator;
use crate::writer::XmlWriter;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Builder for standard XML sitemaps
#[derive(Debug, Default)]
pub struct SitemapBuilder {
    entries: Vec<UrlEntry>,
    validate: bool,
}

impl SitemapBuilder {
    /// Create a new SitemapBuilder
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            validate: true,
        }
    }

    /// Create a new SitemapBuilder with pre-allocated capacity
    ///
    /// This is more efficient when you know the number of URLs in advance.
    ///
    /// # Example
    /// ```
    /// use sitemap_generator::{SitemapBuilder, UrlEntry};
    ///
    /// let mut builder = SitemapBuilder::with_capacity(1000);
    /// for i in 0..1000 {
    ///     builder.add_url(UrlEntry::new(format!("https://example.com/page{}", i)));
    /// }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            validate: true,
        }
    }

    /// Enable or disable validation (enabled by default)
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Add a URL entry
    pub fn add_url(&mut self, entry: UrlEntry) -> &mut Self {
        self.entries.push(entry);
        self
    }

    /// Add multiple URL entries
    pub fn add_urls(&mut self, entries: Vec<UrlEntry>) -> &mut Self {
        self.entries.extend(entries);
        self
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the sitemap is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate all entries
    fn validate_entries(&self) -> Result<()> {
        if !self.validate {
            return Ok(());
        }

        // Check URL count
        Validator::validate_url_count(self.entries.len())?;

        // Validate each entry
        for entry in &self.entries {
            Validator::validate_url(&entry.loc)?;

            if let Some(ref lastmod) = entry.lastmod {
                Validator::validate_date(lastmod)?;
            }

            if let Some(priority) = entry.priority {
                Validator::validate_priority(priority)?;
            }
        }

        Ok(())
    }

    /// Build the sitemap XML as a String
    pub fn build(&self) -> Result<String> {
        self.validate_entries()?;

        let mut writer = XmlWriter::new();
        writer.write_sitemap(&self.entries)?;
        let xml = writer.into_string()?;

        // Validate size
        if self.validate {
            Validator::validate_size(xml.len())?;
        }

        Ok(xml)
    }

    /// Build the sitemap XML as bytes (Vec<u8>)
    ///
    /// This is useful for web frameworks like Axum, Actix-web, etc.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sitemap_generator::{SitemapBuilder, UrlEntry};
    ///
    /// let mut builder = SitemapBuilder::new();
    /// builder.add_url(UrlEntry::new("https://example.com/"));
    /// let bytes = builder.build_bytes().unwrap();
    ///
    /// // Use with Axum:
    /// // Response::builder()
    /// //     .header("Content-Type", "application/xml")
    /// //     .body(bytes)
    /// ```
    pub fn build_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        Ok(xml.into_bytes())
    }

    /// Build the sitemap XML as compressed bytes (gzip)
    ///
    /// This returns gzip-compressed bytes that can be sent directly in HTTP responses.
    /// Remember to set the `Content-Encoding: gzip` header.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sitemap_generator::{SitemapBuilder, UrlEntry};
    ///
    /// let mut builder = SitemapBuilder::new();
    /// builder.add_url(UrlEntry::new("https://example.com/"));
    /// let compressed = builder.build_compressed_bytes().unwrap();
    ///
    /// // Use with Axum:
    /// // Response::builder()
    /// //     .header("Content-Type", "application/xml")
    /// //     .header("Content-Encoding", "gzip")
    /// //     .body(compressed)
    /// ```
    pub fn build_compressed_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(xml.as_bytes())?;
        let compressed = encoder.finish()?;
        Ok(compressed)
    }

    /// Write sitemap to a file
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let mut file = File::create(path)?;
        file.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Write compressed sitemap to a file
    pub fn write_compressed<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(xml.as_bytes())?;
        encoder.finish()?;
        Ok(())
    }
}

/// Builder for image sitemaps
#[derive(Debug, Default)]
pub struct ImageSitemapBuilder {
    entries: Vec<UrlWithImages>,
    validate: bool,
}

impl ImageSitemapBuilder {
    /// Create a new ImageSitemapBuilder
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            validate: true,
        }
    }

    /// Create a new ImageSitemapBuilder with pre-allocated capacity
    ///
    /// # Example
    /// ```
    /// use sitemap_generator::{ImageSitemapBuilder, UrlEntry, UrlWithImages, ImageEntry};
    ///
    /// let mut builder = ImageSitemapBuilder::with_capacity(500);
    /// for i in 0..500 {
    ///     let url = UrlWithImages::new(UrlEntry::new(format!("https://example.com/page{}", i)))
    ///         .add_image(ImageEntry::new(format!("https://example.com/image{}.jpg", i)));
    ///     builder.add_url(url);
    /// }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            validate: true,
        }
    }

    /// Enable or disable validation (enabled by default)
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Add a URL with images
    pub fn add_url(&mut self, entry: UrlWithImages) -> &mut Self {
        self.entries.push(entry);
        self
    }

    /// Add multiple URLs with images
    pub fn add_urls(&mut self, entries: Vec<UrlWithImages>) -> &mut Self {
        self.entries.extend(entries);
        self
    }

    /// Get the number of URL entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the sitemap is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate all entries
    fn validate_entries(&self) -> Result<()> {
        if !self.validate {
            return Ok(());
        }

        // Check URL count
        Validator::validate_url_count(self.entries.len())?;

        // Validate each entry
        for entry in &self.entries {
            Validator::validate_url(&entry.url.loc)?;

            if let Some(ref lastmod) = entry.url.lastmod {
                Validator::validate_date(lastmod)?;
            }

            if let Some(priority) = entry.url.priority {
                Validator::validate_priority(priority)?;
            }

            // Validate image URLs
            for image in &entry.images {
                Validator::validate_url(&image.loc)?;
            }
        }

        Ok(())
    }

    /// Build the sitemap XML as a String
    pub fn build(&self) -> Result<String> {
        self.validate_entries()?;

        let mut writer = XmlWriter::new();
        writer.write_image_sitemap(&self.entries)?;
        let xml = writer.into_string()?;

        // Validate size
        if self.validate {
            Validator::validate_size(xml.len())?;
        }

        Ok(xml)
    }

    /// Build the sitemap XML as bytes (Vec<u8>)
    pub fn build_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        Ok(xml.into_bytes())
    }

    /// Build the sitemap XML as compressed bytes (gzip)
    pub fn build_compressed_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(xml.as_bytes())?;
        let compressed = encoder.finish()?;
        Ok(compressed)
    }

    /// Write sitemap to a file
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let mut file = File::create(path)?;
        file.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Write compressed sitemap to a file
    pub fn write_compressed<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(xml.as_bytes())?;
        encoder.finish()?;
        Ok(())
    }
}

/// Builder for video sitemaps
#[derive(Debug, Default)]
pub struct VideoSitemapBuilder {
    entries: Vec<UrlWithVideos>,
    validate: bool,
}

impl VideoSitemapBuilder {
    /// Create a new VideoSitemapBuilder
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            validate: true,
        }
    }

    /// Create a new VideoSitemapBuilder with pre-allocated capacity
    ///
    /// # Example
    /// ```
    /// use sitemap_generator::{VideoSitemapBuilder, UrlEntry, UrlWithVideos, VideoEntry};
    ///
    /// let mut builder = VideoSitemapBuilder::with_capacity(200);
    /// for i in 0..200 {
    ///     let url = UrlWithVideos::new(UrlEntry::new(format!("https://example.com/video{}", i)))
    ///         .add_video(VideoEntry::new(
    ///             format!("https://example.com/thumb{}.jpg", i),
    ///             format!("Video {}", i),
    ///             "Description"
    ///         ));
    ///     builder.add_url(url);
    /// }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            validate: true,
        }
    }

    /// Enable or disable validation (enabled by default)
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Add a URL with videos
    pub fn add_url(&mut self, entry: UrlWithVideos) -> &mut Self {
        self.entries.push(entry);
        self
    }

    /// Add multiple URLs with videos
    pub fn add_urls(&mut self, entries: Vec<UrlWithVideos>) -> &mut Self {
        self.entries.extend(entries);
        self
    }

    /// Get the number of URL entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the sitemap is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate all entries
    fn validate_entries(&self) -> Result<()> {
        if !self.validate {
            return Ok(());
        }

        // Check URL count
        Validator::validate_url_count(self.entries.len())?;

        // Validate each entry
        for entry in &self.entries {
            Validator::validate_url(&entry.url.loc)?;

            if let Some(ref lastmod) = entry.url.lastmod {
                Validator::validate_date(lastmod)?;
            }

            // Validate videos
            for video in &entry.videos {
                Validator::validate_url(&video.thumbnail_loc)?;
                Validator::validate_video_title(&video.title)?;
                Validator::validate_video_description(&video.description)?;

                if let Some(ref content_loc) = video.content_loc {
                    Validator::validate_url(content_loc)?;
                }

                if let Some(ref player_loc) = video.player_loc {
                    Validator::validate_url(player_loc)?;
                }

                if let Some(duration) = video.duration {
                    Validator::validate_video_duration(duration)?;
                }

                if let Some(rating) = video.rating {
                    Validator::validate_video_rating(rating)?;
                }

                if let Some(ref pub_date) = video.publication_date {
                    Validator::validate_date(pub_date)?;
                }

                if let Some(ref exp_date) = video.expiration_date {
                    Validator::validate_date(exp_date)?;
                }
            }
        }

        Ok(())
    }

    /// Build the sitemap XML as a String
    pub fn build(&self) -> Result<String> {
        self.validate_entries()?;

        let mut writer = XmlWriter::new();
        writer.write_video_sitemap(&self.entries)?;
        let xml = writer.into_string()?;

        // Validate size
        if self.validate {
            Validator::validate_size(xml.len())?;
        }

        Ok(xml)
    }

    /// Build the sitemap XML as bytes (Vec<u8>)
    pub fn build_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        Ok(xml.into_bytes())
    }

    /// Build the sitemap XML as compressed bytes (gzip)
    pub fn build_compressed_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(xml.as_bytes())?;
        let compressed = encoder.finish()?;
        Ok(compressed)
    }

    /// Write sitemap to a file
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let mut file = File::create(path)?;
        file.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Write compressed sitemap to a file
    pub fn write_compressed<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(xml.as_bytes())?;
        encoder.finish()?;
        Ok(())
    }
}

/// Builder for sitemap index
#[derive(Debug, Default)]
pub struct SitemapIndexBuilder {
    entries: Vec<SitemapIndexEntry>,
    validate: bool,
}

impl SitemapIndexBuilder {
    /// Create a new SitemapIndexBuilder
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            validate: true,
        }
    }

    /// Create a new SitemapIndexBuilder with pre-allocated capacity
    ///
    /// # Example
    /// ```
    /// use sitemap_generator::{SitemapIndexBuilder, SitemapIndexEntry};
    ///
    /// let mut builder = SitemapIndexBuilder::with_capacity(50);
    /// for i in 0..50 {
    ///     builder.add_sitemap(
    ///         SitemapIndexEntry::new(format!("https://example.com/sitemap{}.xml.gz", i))
    ///     );
    /// }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            validate: true,
        }
    }

    /// Enable or disable validation (enabled by default)
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Add a sitemap entry
    pub fn add_sitemap(&mut self, entry: SitemapIndexEntry) -> &mut Self {
        self.entries.push(entry);
        self
    }

    /// Add multiple sitemap entries
    pub fn add_sitemaps(&mut self, entries: Vec<SitemapIndexEntry>) -> &mut Self {
        self.entries.extend(entries);
        self
    }

    /// Get the number of sitemap entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the index is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate all entries
    fn validate_entries(&self) -> Result<()> {
        if !self.validate {
            return Ok(());
        }

        // Validate each entry
        for entry in &self.entries {
            Validator::validate_url(&entry.loc)?;

            if let Some(ref lastmod) = entry.lastmod {
                Validator::validate_date(lastmod)?;
            }
        }

        Ok(())
    }

    /// Build the sitemap index XML as a String
    pub fn build(&self) -> Result<String> {
        self.validate_entries()?;

        let mut writer = XmlWriter::new();
        writer.write_sitemap_index(&self.entries)?;
        let xml = writer.into_string()?;

        // Validate size
        if self.validate {
            Validator::validate_size(xml.len())?;
        }

        Ok(xml)
    }

    /// Build the sitemap index XML as bytes (Vec<u8>)
    pub fn build_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        Ok(xml.into_bytes())
    }

    /// Build the sitemap index XML as compressed bytes (gzip)
    pub fn build_compressed_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(xml.as_bytes())?;
        let compressed = encoder.finish()?;
        Ok(compressed)
    }

    /// Write sitemap index to a file
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let mut file = File::create(path)?;
        file.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Write compressed sitemap index to a file
    pub fn write_compressed<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(xml.as_bytes())?;
        encoder.finish()?;
        Ok(())
    }
}

/// Builder for news sitemaps
///
/// News sitemaps are used to submit news articles to Google News.
/// They have a maximum of 1,000 URLs per sitemap and articles should be
/// published within the last 2 days.
pub struct NewsSitemapBuilder {
    entries: Vec<UrlWithNews>,
    validate: bool,
}

impl NewsSitemapBuilder {
    /// Create a new news sitemap builder
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            validate: true,
        }
    }

    /// Create a new news sitemap builder with pre-allocated capacity
    ///
    /// # Example
    /// ```
    /// use sitemap_generator::{NewsSitemapBuilder, UrlEntry, UrlWithNews, NewsEntry, NewsPublication};
    ///
    /// let mut builder = NewsSitemapBuilder::with_capacity(1000);
    /// for i in 0..1000 {
    ///     let publication = NewsPublication::new("News Site", "en");
    ///     let news = NewsEntry::new(publication, "2025-11-01T10:00:00Z", format!("Article {}", i));
    ///     builder.add_url(UrlWithNews::new(UrlEntry::new(format!("https://example.com/news{}", i)), news));
    /// }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            validate: true,
        }
    }

    /// Enable or disable validation (enabled by default)
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Add a URL with news metadata
    pub fn add_url(&mut self, entry: UrlWithNews) -> &mut Self {
        self.entries.push(entry);
        self
    }

    /// Add multiple URLs with news metadata
    pub fn add_urls(&mut self, entries: Vec<UrlWithNews>) -> &mut Self {
        self.entries.extend(entries);
        self
    }

    /// Get the number of URLs
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the sitemap is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate all entries
    fn validate_entries(&self) -> Result<()> {
        if !self.validate {
            return Ok(());
        }

        // Validate URL count (max 1,000 for news sitemaps)
        Validator::validate_news_url_count(self.entries.len())?;

        // Validate each entry
        for entry in &self.entries {
            // Validate URL
            Validator::validate_url(&entry.url.loc)?;

            // Validate publication date
            Validator::validate_date(&entry.news.publication_date)?;

            // Validate language code
            Validator::validate_language_code(&entry.news.publication.language)?;

            // Validate stock tickers if present
            if let Some(ref tickers) = entry.news.stock_tickers {
                Validator::validate_stock_tickers(tickers)?;
            }
        }

        Ok(())
    }

    /// Build the news sitemap XML as a String
    pub fn build(&self) -> Result<String> {
        self.validate_entries()?;

        let mut writer = XmlWriter::new();
        writer.write_news_sitemap(&self.entries)?;
        let xml = writer.into_string()?;

        // Validate size
        if self.validate {
            Validator::validate_size(xml.len())?;
        }

        Ok(xml)
    }

    /// Build the news sitemap XML as bytes (Vec<u8>)
    /// Perfect for web framework responses
    pub fn build_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        Ok(xml.into_bytes())
    }

    /// Build the news sitemap XML as compressed bytes (gzip)
    /// Saves 96-98% bandwidth
    pub fn build_compressed_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(xml.as_bytes())?;
        let compressed = encoder.finish()?;
        Ok(compressed)
    }

    /// Write news sitemap to a file
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let mut file = File::create(path)?;
        file.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Write compressed news sitemap to a file
    pub fn write_compressed<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(xml.as_bytes())?;
        encoder.finish()?;
        Ok(())
    }
}

impl Default for NewsSitemapBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for combined sitemaps with multiple extensions
///
/// Allows combining image, video, and news extensions in a single sitemap.
/// This is useful for pages that have multiple types of content (e.g., news articles with images and videos).
pub struct CombinedSitemapBuilder {
    entries: Vec<UrlWithExtensions>,
    validate: bool,
}

impl CombinedSitemapBuilder {
    /// Create a new combined sitemap builder
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            validate: true,
        }
    }

    /// Create a new combined sitemap builder with pre-allocated capacity
    ///
    /// # Example
    /// ```
    /// use sitemap_generator::{CombinedSitemapBuilder, UrlEntry, UrlWithExtensions, ImageEntry};
    ///
    /// let mut builder = CombinedSitemapBuilder::with_capacity(500);
    /// for i in 0..500 {
    ///     let url = UrlWithExtensions::new(UrlEntry::new(format!("https://example.com/page{}", i)))
    ///         .add_image(ImageEntry::new(format!("https://example.com/img{}.jpg", i)));
    ///     builder.add_url(url);
    /// }
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            validate: true,
        }
    }

    /// Enable or disable validation (enabled by default)
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Add a URL with extensions
    pub fn add_url(&mut self, entry: UrlWithExtensions) -> &mut Self {
        self.entries.push(entry);
        self
    }

    /// Add multiple URLs with extensions
    pub fn add_urls(&mut self, entries: Vec<UrlWithExtensions>) -> &mut Self {
        self.entries.extend(entries);
        self
    }

    /// Get the number of URLs
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the sitemap is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate all entries
    fn validate_entries(&self) -> Result<()> {
        if !self.validate {
            return Ok(());
        }

        // Check if there are any news entries (affects URL limit)
        let has_news = self.entries.iter().any(|e| e.news.is_some());

        // Validate URL count
        if has_news {
            // If any URL has news metadata, apply news sitemap limit (1,000)
            Validator::validate_news_url_count(self.entries.len())?;
        } else {
            // Otherwise use standard sitemap limit (50,000)
            Validator::validate_url_count(self.entries.len())?;
        }

        // Validate each entry
        for entry in &self.entries {
            // Validate URL
            Validator::validate_url(&entry.url.loc)?;

            // Validate URL fields
            if let Some(ref lastmod) = entry.url.lastmod {
                Validator::validate_date(lastmod)?;
            }

            if let Some(priority) = entry.url.priority {
                Validator::validate_priority(priority)?;
            }

            // Validate videos
            for video in &entry.videos {
                Validator::validate_video_title(&video.title)?;
                Validator::validate_video_description(&video.description)?;

                if let Some(duration) = video.duration {
                    Validator::validate_video_duration(duration)?;
                }

                if let Some(rating) = video.rating {
                    Validator::validate_video_rating(rating)?;
                }
            }

            // Validate news (if present)
            if let Some(ref news) = entry.news {
                Validator::validate_date(&news.publication_date)?;
                Validator::validate_language_code(&news.publication.language)?;

                if let Some(ref tickers) = news.stock_tickers {
                    Validator::validate_stock_tickers(tickers)?;
                }
            }
        }

        Ok(())
    }

    /// Build the combined sitemap XML as a String
    pub fn build(&self) -> Result<String> {
        self.validate_entries()?;

        let mut writer = XmlWriter::new();
        writer.write_combined_sitemap(&self.entries)?;
        let xml = writer.into_string()?;

        // Validate size
        if self.validate {
            Validator::validate_size(xml.len())?;
        }

        Ok(xml)
    }

    /// Build the combined sitemap XML as bytes (Vec<u8>)
    /// Perfect for web framework responses
    pub fn build_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        Ok(xml.into_bytes())
    }

    /// Build the combined sitemap XML as compressed bytes (gzip)
    /// Saves 96-98% bandwidth
    pub fn build_compressed_bytes(&self) -> Result<Vec<u8>> {
        let xml = self.build()?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(xml.as_bytes())?;
        let compressed = encoder.finish()?;
        Ok(compressed)
    }

    /// Write combined sitemap to a file
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let mut file = File::create(path)?;
        file.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Write compressed combined sitemap to a file
    pub fn write_compressed<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.build()?;
        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(xml.as_bytes())?;
        encoder.finish()?;
        Ok(())
    }
}

impl Default for CombinedSitemapBuilder {
    fn default() -> Self {
        Self::new()
    }
}
