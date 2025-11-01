//! Validation utilities for sitemap entries

use crate::error::{Error, Result};
use url::Url;

/// Maximum number of URLs allowed in a sitemap
pub const MAX_URLS: usize = 50_000;

/// Maximum size of uncompressed sitemap (50MB)
pub const MAX_SIZE_BYTES: usize = 52_428_800;

/// Maximum URL length
pub const MAX_URL_LENGTH: usize = 2048;

/// Maximum number of URLs allowed in a news sitemap
pub const MAX_NEWS_URLS: usize = 1_000;

/// Validator for sitemap entries
pub struct Validator;

impl Validator {
    /// Validate a URL
    pub fn validate_url(url: &str) -> Result<()> {
        // Check length
        if url.len() > MAX_URL_LENGTH {
            return Err(Error::UrlTooLong(url.to_string()));
        }

        // Parse URL to ensure it's valid
        Url::parse(url).map_err(|_| Error::InvalidUrl(url.to_string()))?;

        Ok(())
    }

    /// Validate priority value (must be 0.0 to 1.0)
    pub fn validate_priority(priority: f32) -> Result<()> {
        if !(0.0..=1.0).contains(&priority) {
            return Err(Error::InvalidPriority(priority));
        }
        Ok(())
    }

    /// Validate date format (W3C Datetime)
    pub fn validate_date(date: &str) -> Result<()> {
        // Basic validation for W3C Datetime format
        // Accept YYYY-MM-DD or full ISO 8601 datetime
        if date.len() < 10 {
            return Err(Error::InvalidDate(date.to_string()));
        }

        // Check basic format YYYY-MM-DD
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() < 3 {
            return Err(Error::InvalidDate(date.to_string()));
        }

        // Validate year, month, day are numeric
        if parts[0].len() != 4 || !parts[0].chars().all(|c| c.is_numeric()) {
            return Err(Error::InvalidDate(date.to_string()));
        }
        if parts[1].len() != 2 || !parts[1].chars().all(|c| c.is_numeric()) {
            return Err(Error::InvalidDate(date.to_string()));
        }

        // Day part might contain time, so just check first 2 chars
        if parts[2].len() < 2 || !parts[2].chars().take(2).all(|c| c.is_numeric()) {
            return Err(Error::InvalidDate(date.to_string()));
        }

        Ok(())
    }

    /// Validate number of URLs doesn't exceed maximum
    pub fn validate_url_count(count: usize) -> Result<()> {
        if count > MAX_URLS {
            return Err(Error::TooManyUrls(count));
        }
        Ok(())
    }

    /// Validate sitemap size doesn't exceed maximum
    pub fn validate_size(size: usize) -> Result<()> {
        if size > MAX_SIZE_BYTES {
            return Err(Error::SizeExceeded(size));
        }
        Ok(())
    }

    /// Validate video duration (0 to 28800 seconds = 8 hours)
    pub fn validate_video_duration(duration: u32) -> Result<()> {
        if duration > 28_800 {
            return Err(Error::Validation(format!(
                "Video duration exceeds maximum (28800 seconds): {}",
                duration
            )));
        }
        Ok(())
    }

    /// Validate video rating (0.0 to 5.0)
    pub fn validate_video_rating(rating: f32) -> Result<()> {
        if !(0.0..=5.0).contains(&rating) {
            return Err(Error::Validation(format!(
                "Video rating must be between 0.0 and 5.0: {}",
                rating
            )));
        }
        Ok(())
    }

    /// Validate video title length (max 100 characters)
    pub fn validate_video_title(title: &str) -> Result<()> {
        if title.len() > 100 {
            return Err(Error::Validation(format!(
                "Video title exceeds 100 characters: {}",
                title.len()
            )));
        }
        Ok(())
    }

    /// Validate video description length (max 2048 characters)
    pub fn validate_video_description(description: &str) -> Result<()> {
        if description.len() > 2048 {
            return Err(Error::Validation(format!(
                "Video description exceeds 2048 characters: {}",
                description.len()
            )));
        }
        Ok(())
    }

    /// Validate news sitemap URL count (max 1,000 URLs)
    pub fn validate_news_url_count(count: usize) -> Result<()> {
        if count > MAX_NEWS_URLS {
            return Err(Error::Validation(format!(
                "News sitemap exceeds maximum of {} URLs: {}",
                MAX_NEWS_URLS, count
            )));
        }
        Ok(())
    }

    /// Validate language code (ISO 639 format)
    pub fn validate_language_code(code: &str) -> Result<()> {
        let len = code.len();
        // Accept 2-3 letter codes, or special Chinese variants
        if (len == 2 || len == 3) && code.chars().all(|c| c.is_ascii_lowercase()) {
            return Ok(());
        }
        // Special cases for Chinese
        if code == "zh-cn" || code == "zh-tw" {
            return Ok(());
        }
        Err(Error::Validation(format!(
            "Invalid language code (must be ISO 639 2-3 letters or zh-cn/zh-tw): {}",
            code
        )))
    }

    /// Validate stock tickers (max 5, comma-separated)
    pub fn validate_stock_tickers(tickers: &str) -> Result<()> {
        let ticker_list: Vec<&str> = tickers.split(',').map(|s| s.trim()).collect();
        if ticker_list.len() > 5 {
            return Err(Error::Validation(format!(
                "Stock tickers exceed maximum of 5: {}",
                ticker_list.len()
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url() {
        assert!(Validator::validate_url("https://example.com").is_ok());
        assert!(Validator::validate_url("https://example.com/page").is_ok());
        assert!(Validator::validate_url("not a url").is_err());

        // Test URL too long
        let long_url = format!("https://example.com/{}", "a".repeat(2100));
        assert!(Validator::validate_url(&long_url).is_err());
    }

    #[test]
    fn test_validate_priority() {
        assert!(Validator::validate_priority(0.0).is_ok());
        assert!(Validator::validate_priority(0.5).is_ok());
        assert!(Validator::validate_priority(1.0).is_ok());
        assert!(Validator::validate_priority(-0.1).is_err());
        assert!(Validator::validate_priority(1.1).is_err());
    }

    #[test]
    fn test_validate_date() {
        assert!(Validator::validate_date("2025-11-01").is_ok());
        assert!(Validator::validate_date("2024-12-31").is_ok());
        assert!(Validator::validate_date("2025-11-01T12:00:00Z").is_ok());
        assert!(Validator::validate_date("invalid").is_err());
        assert!(Validator::validate_date("24-01-01").is_err());
    }

    #[test]
    fn test_validate_url_count() {
        assert!(Validator::validate_url_count(1000).is_ok());
        assert!(Validator::validate_url_count(50_000).is_ok());
        assert!(Validator::validate_url_count(50_001).is_err());
    }

    #[test]
    fn test_validate_video_rating() {
        assert!(Validator::validate_video_rating(0.0).is_ok());
        assert!(Validator::validate_video_rating(2.5).is_ok());
        assert!(Validator::validate_video_rating(5.0).is_ok());
        assert!(Validator::validate_video_rating(-0.1).is_err());
        assert!(Validator::validate_video_rating(5.1).is_err());
    }

    #[test]
    fn test_validate_news_url_count() {
        assert!(Validator::validate_news_url_count(100).is_ok());
        assert!(Validator::validate_news_url_count(1_000).is_ok());
        assert!(Validator::validate_news_url_count(1_001).is_err());
    }

    #[test]
    fn test_validate_language_code() {
        // Valid 2-letter codes
        assert!(Validator::validate_language_code("en").is_ok());
        assert!(Validator::validate_language_code("vi").is_ok());
        assert!(Validator::validate_language_code("fr").is_ok());

        // Valid 3-letter codes
        assert!(Validator::validate_language_code("eng").is_ok());
        assert!(Validator::validate_language_code("vie").is_ok());

        // Special Chinese variants
        assert!(Validator::validate_language_code("zh-cn").is_ok());
        assert!(Validator::validate_language_code("zh-tw").is_ok());

        // Invalid codes
        assert!(Validator::validate_language_code("EN").is_err()); // uppercase
        assert!(Validator::validate_language_code("e").is_err()); // too short
        assert!(Validator::validate_language_code("engl").is_err()); // too long
        assert!(Validator::validate_language_code("en-us").is_err()); // not allowed
    }

    #[test]
    fn test_validate_stock_tickers() {
        assert!(Validator::validate_stock_tickers("AAPL").is_ok());
        assert!(Validator::validate_stock_tickers("AAPL,GOOGL,MSFT").is_ok());
        assert!(Validator::validate_stock_tickers("AAPL, GOOGL, MSFT, TSLA, AMZN").is_ok());
        assert!(Validator::validate_stock_tickers("AAPL,GOOGL,MSFT,TSLA,AMZN,FB").is_err()); // 6 tickers
    }
}
