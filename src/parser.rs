//! Parser for reading sitemap XML files

use crate::error::{Error, Result};
use crate::types::*;
use flate2::read::GzDecoder;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Parser for sitemap XML files
pub struct SitemapParser;

impl SitemapParser {
    /// Parse a sitemap from a string
    pub fn parse_string(xml: &str) -> Result<Vec<UrlEntry>> {
        Self::parse_reader(xml.as_bytes())
    }

    /// Parse a sitemap from a file
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Vec<UrlEntry>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Self::parse_reader(reader)
    }

    /// Parse a compressed sitemap from a file
    pub fn parse_compressed<P: AsRef<Path>>(path: P) -> Result<Vec<UrlEntry>> {
        let file = File::open(path)?;
        let decoder = GzDecoder::new(file);
        let reader = BufReader::new(decoder);
        Self::parse_reader(reader)
    }

    /// Parse a sitemap from a reader
    fn parse_reader<R: BufRead>(reader: R) -> Result<Vec<UrlEntry>> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        let mut entries = Vec::new();
        let mut current_url: Option<UrlEntry> = None;
        let mut current_element = String::new();
        let mut buf = Vec::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if name == "url" {
                        current_url = Some(UrlEntry {
                            loc: String::new(),
                            lastmod: None,
                            changefreq: None,
                            priority: None,
                        });
                    }
                    current_element = name;
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().map_err(|e| Error::Xml(e.to_string()))?;
                    let text = text.to_string();

                    if let Some(ref mut url) = current_url {
                        match current_element.as_str() {
                            "loc" => url.loc = text,
                            "lastmod" => url.lastmod = Some(text),
                            "changefreq" => {
                                url.changefreq = Some(match text.as_str() {
                                    "always" => ChangeFreq::Always,
                                    "hourly" => ChangeFreq::Hourly,
                                    "daily" => ChangeFreq::Daily,
                                    "weekly" => ChangeFreq::Weekly,
                                    "monthly" => ChangeFreq::Monthly,
                                    "yearly" => ChangeFreq::Yearly,
                                    "never" => ChangeFreq::Never,
                                    _ => {
                                        return Err(Error::InvalidChangeFreq(text));
                                    }
                                });
                            }
                            "priority" => {
                                url.priority = Some(
                                    text.parse::<f32>()
                                        .map_err(|_| Error::Validation(format!("Invalid priority: {}", text)))?,
                                );
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = String::from_utf8_lossy(name.as_ref());
                    if name_str == "url" {
                        if let Some(url) = current_url.take() {
                            entries.push(url);
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::Xml(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(entries)
    }

    /// Parse a sitemap index from a string
    pub fn parse_index_string(xml: &str) -> Result<Vec<SitemapIndexEntry>> {
        Self::parse_index_reader(xml.as_bytes())
    }

    /// Parse a sitemap index from a file
    pub fn parse_index_file<P: AsRef<Path>>(path: P) -> Result<Vec<SitemapIndexEntry>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Self::parse_index_reader(reader)
    }

    /// Parse a compressed sitemap index from a file
    pub fn parse_index_compressed<P: AsRef<Path>>(path: P) -> Result<Vec<SitemapIndexEntry>> {
        let file = File::open(path)?;
        let decoder = GzDecoder::new(file);
        let reader = BufReader::new(decoder);
        Self::parse_index_reader(reader)
    }

    /// Parse a sitemap index from a reader
    fn parse_index_reader<R: BufRead>(reader: R) -> Result<Vec<SitemapIndexEntry>> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        let mut entries = Vec::new();
        let mut current_sitemap: Option<SitemapIndexEntry> = None;
        let mut current_element = String::new();
        let mut buf = Vec::new();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if name == "sitemap" {
                        current_sitemap = Some(SitemapIndexEntry {
                            loc: String::new(),
                            lastmod: None,
                        });
                    }
                    current_element = name;
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().map_err(|e| Error::Xml(e.to_string()))?;
                    let text = text.to_string();

                    if let Some(ref mut sitemap) = current_sitemap {
                        match current_element.as_str() {
                            "loc" => sitemap.loc = text,
                            "lastmod" => sitemap.lastmod = Some(text),
                            _ => {}
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = String::from_utf8_lossy(name.as_ref());
                    if name_str == "sitemap" {
                        if let Some(sitemap) = current_sitemap.take() {
                            entries.push(sitemap);
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::Xml(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_sitemap() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://example.com/</loc>
    <lastmod>2025-11-01</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://example.com/page1</loc>
    <lastmod>2025-11-02</lastmod>
    <priority>0.8</priority>
  </url>
</urlset>"#;

        let entries = SitemapParser::parse_string(xml).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].loc, "https://example.com/");
        assert_eq!(entries[0].lastmod, Some("2025-11-01".to_string()));
        assert_eq!(entries[0].changefreq, Some(ChangeFreq::Daily));
        assert_eq!(entries[0].priority, Some(1.0));

        assert_eq!(entries[1].loc, "https://example.com/page1");
        assert_eq!(entries[1].lastmod, Some("2025-11-02".to_string()));
        assert_eq!(entries[1].priority, Some(0.8));
    }

    #[test]
    fn test_parse_sitemap_index() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sitemap>
    <loc>https://example.com/sitemap1.xml</loc>
    <lastmod>2025-11-01</lastmod>
  </sitemap>
  <sitemap>
    <loc>https://example.com/sitemap2.xml</loc>
    <lastmod>2025-11-02</lastmod>
  </sitemap>
</sitemapindex>"#;

        let entries = SitemapParser::parse_index_string(xml).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].loc, "https://example.com/sitemap1.xml");
        assert_eq!(entries[0].lastmod, Some("2025-11-01".to_string()));

        assert_eq!(entries[1].loc, "https://example.com/sitemap2.xml");
        assert_eq!(entries[1].lastmod, Some("2025-11-02".to_string()));
    }
}
