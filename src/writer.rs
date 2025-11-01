//! XML writer for sitemaps

use crate::error::{Error, Result};
use crate::types::*;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;

/// XML namespaces
const SITEMAP_NS: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";
const IMAGE_NS: &str = "http://www.google.com/schemas/sitemap-image/1.1";
const VIDEO_NS: &str = "http://www.google.com/schemas/sitemap-video/1.1";
const NEWS_NS: &str = "http://www.google.com/schemas/sitemap-news/0.9";

/// Writer for generating sitemap XML
pub struct XmlWriter {
    writer: Writer<Cursor<Vec<u8>>>,
}

impl XmlWriter {
    /// Create a new XmlWriter
    pub fn new() -> Self {
        let cursor = Cursor::new(Vec::with_capacity(8192)); // Pre-allocate 8KB
        let writer = Writer::new_with_indent(cursor, b' ', 2);

        Self { writer }
    }

    /// Get the generated XML as a String
    pub fn into_string(self) -> Result<String> {
        let result = self.writer.into_inner().into_inner();
        String::from_utf8(result).map_err(|e| Error::Xml(e.to_string()))
    }

    /// Write XML declaration
    fn write_declaration(&mut self) -> Result<()> {
        self.writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
        Ok(())
    }

    /// Escape XML special characters
    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    /// Write a simple text element
    fn write_text_element(&mut self, name: &str, text: &str) -> Result<()> {
        self.writer
            .write_event(Event::Start(BytesStart::new(name)))?;
        self.writer
            .write_event(Event::Text(BytesText::new(&Self::escape_xml(text))))?;
        self.writer
            .write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }

    /// Write a standard sitemap
    pub fn write_sitemap(&mut self, entries: &[UrlEntry]) -> Result<()> {
        self.write_declaration()?;

        // Start urlset
        let mut urlset = BytesStart::new("urlset");
        urlset.push_attribute(("xmlns", SITEMAP_NS));
        self.writer.write_event(Event::Start(urlset))?;

        // Write each URL entry
        for entry in entries {
            self.write_url_entry(entry)?;
        }

        // End urlset
        self.writer
            .write_event(Event::End(BytesEnd::new("urlset")))?;

        Ok(())
    }

    /// Write a single URL entry
    fn write_url_entry(&mut self, entry: &UrlEntry) -> Result<()> {
        self.writer
            .write_event(Event::Start(BytesStart::new("url")))?;

        self.write_text_element("loc", &entry.loc)?;

        if let Some(ref lastmod) = entry.lastmod {
            self.write_text_element("lastmod", lastmod)?;
        }

        if let Some(changefreq) = entry.changefreq {
            self.write_text_element("changefreq", changefreq.as_str())?;
        }

        if let Some(priority) = entry.priority {
            self.write_text_element("priority", &priority.to_string())?;
        }

        self.writer
            .write_event(Event::End(BytesEnd::new("url")))?;

        Ok(())
    }

    /// Write an image sitemap
    pub fn write_image_sitemap(&mut self, entries: &[UrlWithImages]) -> Result<()> {
        self.write_declaration()?;

        // Start urlset with image namespace
        let mut urlset = BytesStart::new("urlset");
        urlset.push_attribute(("xmlns", SITEMAP_NS));
        urlset.push_attribute(("xmlns:image", IMAGE_NS));
        self.writer.write_event(Event::Start(urlset))?;

        // Write each URL with images
        for entry in entries {
            self.write_url_with_images(entry)?;
        }

        // End urlset
        self.writer
            .write_event(Event::End(BytesEnd::new("urlset")))?;

        Ok(())
    }

    /// Write a URL entry with images
    fn write_url_with_images(&mut self, entry: &UrlWithImages) -> Result<()> {
        self.writer
            .write_event(Event::Start(BytesStart::new("url")))?;

        self.write_text_element("loc", &entry.url.loc)?;

        if let Some(ref lastmod) = entry.url.lastmod {
            self.write_text_element("lastmod", lastmod)?;
        }

        if let Some(changefreq) = entry.url.changefreq {
            self.write_text_element("changefreq", changefreq.as_str())?;
        }

        if let Some(priority) = entry.url.priority {
            self.write_text_element("priority", &priority.to_string())?;
        }

        // Write images
        for image in &entry.images {
            self.write_image_entry(image)?;
        }

        self.writer
            .write_event(Event::End(BytesEnd::new("url")))?;

        Ok(())
    }

    /// Write an image entry
    fn write_image_entry(&mut self, image: &ImageEntry) -> Result<()> {
        self.writer
            .write_event(Event::Start(BytesStart::new("image:image")))?;

        self.write_text_element("image:loc", &image.loc)?;

        if let Some(ref caption) = image.caption {
            self.write_text_element("image:caption", caption)?;
        }

        if let Some(ref geo) = image.geo_location {
            if let Some(ref city) = geo.city {
                self.write_text_element("image:geo_location", city)?;
            } else if let Some(ref state) = geo.state {
                self.write_text_element("image:geo_location", state)?;
            } else if let Some(ref country) = geo.country {
                self.write_text_element("image:geo_location", country)?;
            }
        }

        if let Some(ref title) = image.title {
            self.write_text_element("image:title", title)?;
        }

        if let Some(ref license) = image.license {
            self.write_text_element("image:license", license)?;
        }

        self.writer
            .write_event(Event::End(BytesEnd::new("image:image")))?;

        Ok(())
    }

    /// Write a video sitemap
    pub fn write_video_sitemap(&mut self, entries: &[UrlWithVideos]) -> Result<()> {
        self.write_declaration()?;

        // Start urlset with video namespace
        let mut urlset = BytesStart::new("urlset");
        urlset.push_attribute(("xmlns", SITEMAP_NS));
        urlset.push_attribute(("xmlns:video", VIDEO_NS));
        self.writer.write_event(Event::Start(urlset))?;

        // Write each URL with videos
        for entry in entries {
            self.write_url_with_videos(entry)?;
        }

        // End urlset
        self.writer
            .write_event(Event::End(BytesEnd::new("urlset")))?;

        Ok(())
    }

    /// Write a URL entry with videos
    fn write_url_with_videos(&mut self, entry: &UrlWithVideos) -> Result<()> {
        self.writer
            .write_event(Event::Start(BytesStart::new("url")))?;

        self.write_text_element("loc", &entry.url.loc)?;

        if let Some(ref lastmod) = entry.url.lastmod {
            self.write_text_element("lastmod", lastmod)?;
        }

        // Write videos
        for video in &entry.videos {
            self.write_video_entry(video)?;
        }

        self.writer
            .write_event(Event::End(BytesEnd::new("url")))?;

        Ok(())
    }

    /// Write a video entry
    fn write_video_entry(&mut self, video: &VideoEntry) -> Result<()> {
        self.writer
            .write_event(Event::Start(BytesStart::new("video:video")))?;

        self.write_text_element("video:thumbnail_loc", &video.thumbnail_loc)?;
        self.write_text_element("video:title", &video.title)?;
        self.write_text_element("video:description", &video.description)?;

        if let Some(ref content_loc) = video.content_loc {
            self.write_text_element("video:content_loc", content_loc)?;
        }

        if let Some(ref player_loc) = video.player_loc {
            self.write_text_element("video:player_loc", player_loc)?;
        }

        if let Some(duration) = video.duration {
            self.write_text_element("video:duration", &duration.to_string())?;
        }

        if let Some(ref pub_date) = video.publication_date {
            self.write_text_element("video:publication_date", pub_date)?;
        }

        if let Some(ref exp_date) = video.expiration_date {
            self.write_text_element("video:expiration_date", exp_date)?;
        }

        if let Some(rating) = video.rating {
            self.write_text_element("video:rating", &rating.to_string())?;
        }

        if let Some(view_count) = video.view_count {
            self.write_text_element("video:view_count", &view_count.to_string())?;
        }

        if let Some(family_friendly) = video.family_friendly {
            let value = if family_friendly { "yes" } else { "no" };
            self.write_text_element("video:family_friendly", value)?;
        }

        for tag in &video.tags {
            self.write_text_element("video:tag", tag)?;
        }

        if let Some(ref category) = video.category {
            self.write_text_element("video:category", category)?;
        }

        if let Some(ref restriction) = video.restriction {
            match restriction {
                VideoCountryRestriction::Allow(countries) => {
                    let mut elem = BytesStart::new("video:restriction");
                    elem.push_attribute(("relationship", "allow"));
                    self.writer.write_event(Event::Start(elem))?;
                    self.writer
                        .write_event(Event::Text(BytesText::new(&countries.join(" "))))?;
                    self.writer
                        .write_event(Event::End(BytesEnd::new("video:restriction")))?;
                }
                VideoCountryRestriction::Deny(countries) => {
                    let mut elem = BytesStart::new("video:restriction");
                    elem.push_attribute(("relationship", "deny"));
                    self.writer.write_event(Event::Start(elem))?;
                    self.writer
                        .write_event(Event::Text(BytesText::new(&countries.join(" "))))?;
                    self.writer
                        .write_event(Event::End(BytesEnd::new("video:restriction")))?;
                }
            }
        }

        if let Some(ref gallery_loc) = video.gallery_loc {
            self.write_text_element("video:gallery_loc", gallery_loc)?;
        }

        if let Some(ref platform) = video.platform {
            match platform {
                VideoPlatformRestriction::Allow(platforms) => {
                    let mut elem = BytesStart::new("video:platform");
                    elem.push_attribute(("relationship", "allow"));
                    self.writer.write_event(Event::Start(elem))?;
                    let platforms_str: Vec<&str> = platforms.iter().map(|p| p.as_str()).collect();
                    self.writer
                        .write_event(Event::Text(BytesText::new(&platforms_str.join(" "))))?;
                    self.writer
                        .write_event(Event::End(BytesEnd::new("video:platform")))?;
                }
                VideoPlatformRestriction::Deny(platforms) => {
                    let mut elem = BytesStart::new("video:platform");
                    elem.push_attribute(("relationship", "deny"));
                    self.writer.write_event(Event::Start(elem))?;
                    let platforms_str: Vec<&str> = platforms.iter().map(|p| p.as_str()).collect();
                    self.writer
                        .write_event(Event::Text(BytesText::new(&platforms_str.join(" "))))?;
                    self.writer
                        .write_event(Event::End(BytesEnd::new("video:platform")))?;
                }
            }
        }

        for price in &video.prices {
            let mut elem = BytesStart::new("video:price");
            elem.push_attribute(("currency", price.currency.as_str()));
            if let Some(ref resolution) = price.resolution {
                elem.push_attribute(("resolution", resolution.as_str()));
            }
            if let Some(ref type_) = price.type_ {
                elem.push_attribute(("type", type_.as_str()));
            }
            self.writer.write_event(Event::Start(elem))?;
            self.writer
                .write_event(Event::Text(BytesText::new(&price.amount.to_string())))?;
            self.writer
                .write_event(Event::End(BytesEnd::new("video:price")))?;
        }

        if let Some(requires_sub) = video.requires_subscription {
            self.write_text_element("video:requires_subscription", requires_sub.as_str())?;
        }

        if let Some(ref uploader) = video.uploader {
            if let Some(ref info_url) = uploader.info_url {
                let mut elem = BytesStart::new("video:uploader");
                elem.push_attribute(("info", info_url.as_str()));
                self.writer.write_event(Event::Start(elem))?;
                self.writer
                    .write_event(Event::Text(BytesText::new(&uploader.name)))?;
                self.writer
                    .write_event(Event::End(BytesEnd::new("video:uploader")))?;
            } else {
                self.write_text_element("video:uploader", &uploader.name)?;
            }
        }

        if let Some(live) = video.live {
            self.write_text_element("video:live", live.as_str())?;
        }

        self.writer
            .write_event(Event::End(BytesEnd::new("video:video")))?;

        Ok(())
    }

    /// Write a news sitemap
    pub fn write_news_sitemap(&mut self, entries: &[UrlWithNews]) -> Result<()> {
        self.write_declaration()?;

        // Start urlset with news namespace
        let mut urlset = BytesStart::new("urlset");
        urlset.push_attribute(("xmlns", SITEMAP_NS));
        urlset.push_attribute(("xmlns:news", NEWS_NS));
        self.writer.write_event(Event::Start(urlset))?;

        // Write each URL entry with news
        for entry in entries {
            self.writer
                .write_event(Event::Start(BytesStart::new("url")))?;

            // Write URL loc
            self.write_text_element("loc", &entry.url.loc)?;

            // Write news metadata
            self.writer
                .write_event(Event::Start(BytesStart::new("news:news")))?;

            // Write publication
            self.writer
                .write_event(Event::Start(BytesStart::new("news:publication")))?;
            self.write_text_element("news:name", &entry.news.publication.name)?;
            self.write_text_element("news:language", &entry.news.publication.language)?;
            self.writer
                .write_event(Event::End(BytesEnd::new("news:publication")))?;

            // Write publication_date
            self.write_text_element("news:publication_date", &entry.news.publication_date)?;

            // Write title
            self.write_text_element("news:title", &entry.news.title)?;

            // Write optional keywords
            if let Some(ref keywords) = entry.news.keywords {
                self.write_text_element("news:keywords", keywords)?;
            }

            // Write optional stock_tickers
            if let Some(ref tickers) = entry.news.stock_tickers {
                self.write_text_element("news:stock_tickers", tickers)?;
            }

            // End news:news
            self.writer
                .write_event(Event::End(BytesEnd::new("news:news")))?;

            // End url
            self.writer
                .write_event(Event::End(BytesEnd::new("url")))?;
        }

        // End urlset
        self.writer
            .write_event(Event::End(BytesEnd::new("urlset")))?;

        Ok(())
    }

    /// Write a combined sitemap (with multiple extensions)
    /// Supports combining image, video, and news extensions in a single sitemap
    pub fn write_combined_sitemap(&mut self, entries: &[UrlWithExtensions]) -> Result<()> {
        self.write_declaration()?;

        // Start urlset with all necessary namespaces
        let mut urlset = BytesStart::new("urlset");
        urlset.push_attribute(("xmlns", SITEMAP_NS));

        // Check which namespaces are needed
        let needs_image = entries.iter().any(|e| !e.images.is_empty());
        let needs_video = entries.iter().any(|e| !e.videos.is_empty());
        let needs_news = entries.iter().any(|e| e.news.is_some());

        if needs_image {
            urlset.push_attribute(("xmlns:image", IMAGE_NS));
        }
        if needs_video {
            urlset.push_attribute(("xmlns:video", VIDEO_NS));
        }
        if needs_news {
            urlset.push_attribute(("xmlns:news", NEWS_NS));
        }

        self.writer.write_event(Event::Start(urlset))?;

        // Write each URL with its extensions
        for entry in entries {
            self.writer
                .write_event(Event::Start(BytesStart::new("url")))?;

            // Write URL loc
            self.write_text_element("loc", &entry.url.loc)?;

            // Write optional URL fields
            if let Some(ref lastmod) = entry.url.lastmod {
                self.write_text_element("lastmod", lastmod)?;
            }

            if let Some(changefreq) = entry.url.changefreq {
                self.write_text_element("changefreq", changefreq.as_str())?;
            }

            if let Some(priority) = entry.url.priority {
                self.write_text_element("priority", &priority.to_string())?;
            }

            // Write images
            for image in &entry.images {
                self.write_image_entry(image)?;
            }

            // Write videos
            for video in &entry.videos {
                self.write_video_entry(video)?;
            }

            // Write news (max one per URL)
            if let Some(ref news) = entry.news {
                self.writer
                    .write_event(Event::Start(BytesStart::new("news:news")))?;

                // Write publication
                self.writer
                    .write_event(Event::Start(BytesStart::new("news:publication")))?;
                self.write_text_element("news:name", &news.publication.name)?;
                self.write_text_element("news:language", &news.publication.language)?;
                self.writer
                    .write_event(Event::End(BytesEnd::new("news:publication")))?;

                // Write publication_date
                self.write_text_element("news:publication_date", &news.publication_date)?;

                // Write title
                self.write_text_element("news:title", &news.title)?;

                // Write optional keywords
                if let Some(ref keywords) = news.keywords {
                    self.write_text_element("news:keywords", keywords)?;
                }

                // Write optional stock_tickers
                if let Some(ref tickers) = news.stock_tickers {
                    self.write_text_element("news:stock_tickers", tickers)?;
                }

                // End news:news
                self.writer
                    .write_event(Event::End(BytesEnd::new("news:news")))?;
            }

            // End url
            self.writer
                .write_event(Event::End(BytesEnd::new("url")))?;
        }

        // End urlset
        self.writer
            .write_event(Event::End(BytesEnd::new("urlset")))?;

        Ok(())
    }

    /// Write a sitemap index
    pub fn write_sitemap_index(&mut self, entries: &[SitemapIndexEntry]) -> Result<()> {
        self.write_declaration()?;

        // Start sitemapindex
        let mut sitemapindex = BytesStart::new("sitemapindex");
        sitemapindex.push_attribute(("xmlns", SITEMAP_NS));
        self.writer.write_event(Event::Start(sitemapindex))?;

        // Write each sitemap entry
        for entry in entries {
            self.writer
                .write_event(Event::Start(BytesStart::new("sitemap")))?;

            self.write_text_element("loc", &entry.loc)?;

            if let Some(ref lastmod) = entry.lastmod {
                self.write_text_element("lastmod", lastmod)?;
            }

            self.writer
                .write_event(Event::End(BytesEnd::new("sitemap")))?;
        }

        // End sitemapindex
        self.writer
            .write_event(Event::End(BytesEnd::new("sitemapindex")))?;

        Ok(())
    }
}

impl Default for XmlWriter {
    fn default() -> Self {
        Self::new()
    }
}
