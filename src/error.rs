//! Error types for sitemap generation

use std::fmt;
use std::io;

/// Result type alias for sitemap operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur during sitemap generation
#[derive(Debug)]
pub enum Error {
    /// Invalid URL format
    InvalidUrl(String),

    /// URL exceeds maximum length (2048 characters)
    UrlTooLong(String),

    /// Too many URLs in sitemap (>50,000)
    TooManyUrls(usize),

    /// Sitemap size exceeds maximum (50MB uncompressed)
    SizeExceeded(usize),

    /// Invalid date format (must be W3C Datetime format)
    InvalidDate(String),

    /// Invalid priority value (must be 0.0-1.0)
    InvalidPriority(f32),

    /// Invalid changefreq value
    InvalidChangeFreq(String),

    /// IO error during file operations
    Io(io::Error),

    /// XML writing/parsing error
    Xml(String),

    /// Compression error
    Compression(String),

    /// Validation error
    Validation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            Error::UrlTooLong(url) => write!(f, "URL too long (max 2048 chars): {}", url),
            Error::TooManyUrls(count) => write!(f, "Too many URLs in sitemap (max 50,000): {}", count),
            Error::SizeExceeded(size) => write!(f, "Sitemap size exceeds 50MB: {} bytes", size),
            Error::InvalidDate(date) => write!(f, "Invalid date format: {}", date),
            Error::InvalidPriority(priority) => write!(f, "Invalid priority (must be 0.0-1.0): {}", priority),
            Error::InvalidChangeFreq(freq) => write!(f, "Invalid changefreq: {}", freq),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Xml(msg) => write!(f, "XML error: {}", msg),
            Error::Compression(msg) => write!(f, "Compression error: {}", msg),
            Error::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        Error::Xml(err.to_string())
    }
}
