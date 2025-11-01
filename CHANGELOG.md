# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-01

### Added

- Initial release
- Standard XML sitemap generation
- Image sitemap support with metadata
- Video sitemap support with comprehensive metadata
- Sitemap index support for managing multiple sitemaps
- Built-in validation for URLs, dates, priorities, and other fields
- Gzip compression support
- Parser for reading existing sitemap files
- Comprehensive examples for all sitemap types
- Full documentation with rustdoc
- Zero unsafe code

### Features

- **High Performance**: Optimized XML generation with `quick-xml`
- **Memory Efficient**: Pre-allocated buffers and minimal cloning
- **Protocol Compliant**: Follows sitemaps.org protocol 0.9
- **Validation**: Automatic validation with option to disable
- **Compression**: Built-in gzip compression
- **Parsing**: Read and parse existing sitemaps

[0.1.0]: https://github.com/yourusername/sitemap_generator/releases/tag/v0.1.0
