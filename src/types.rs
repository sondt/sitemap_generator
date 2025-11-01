//! Type definitions for sitemap entries

/// How frequently the page is likely to change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeFreq {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl ChangeFreq {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangeFreq::Always => "always",
            ChangeFreq::Hourly => "hourly",
            ChangeFreq::Daily => "daily",
            ChangeFreq::Weekly => "weekly",
            ChangeFreq::Monthly => "monthly",
            ChangeFreq::Yearly => "yearly",
            ChangeFreq::Never => "never",
        }
    }
}

impl std::fmt::Display for ChangeFreq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A standard URL entry in a sitemap
#[derive(Debug, Clone)]
pub struct UrlEntry {
    /// The URL of the page (required)
    pub loc: String,

    /// The date of last modification (W3C Datetime format)
    pub lastmod: Option<String>,

    /// How frequently the page is likely to change
    pub changefreq: Option<ChangeFreq>,

    /// The priority of this URL relative to other URLs (0.0 to 1.0)
    pub priority: Option<f32>,
}

impl UrlEntry {
    /// Create a new URL entry
    pub fn new(loc: impl Into<String>) -> Self {
        Self {
            loc: loc.into(),
            lastmod: None,
            changefreq: None,
            priority: None,
        }
    }

    /// Set the last modification date (W3C Datetime format: YYYY-MM-DD)
    pub fn lastmod(mut self, lastmod: impl Into<String>) -> Self {
        self.lastmod = Some(lastmod.into());
        self
    }

    /// Set the change frequency
    pub fn changefreq(mut self, changefreq: ChangeFreq) -> Self {
        self.changefreq = Some(changefreq);
        self
    }

    /// Set the priority (0.0 to 1.0)
    pub fn priority(mut self, priority: f32) -> Self {
        self.priority = Some(priority);
        self
    }
}

/// Geographic location for an image
#[derive(Debug, Clone)]
pub struct GeoLocation {
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

impl GeoLocation {
    pub fn new() -> Self {
        Self {
            city: None,
            state: None,
            country: None,
        }
    }

    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
}

/// An image entry in an image sitemap
#[derive(Debug, Clone)]
pub struct ImageEntry {
    /// The URL of the image (required)
    pub loc: String,

    /// A caption for the image
    pub caption: Option<String>,

    /// The geographic location of the image
    pub geo_location: Option<GeoLocation>,

    /// The title of the image
    pub title: Option<String>,

    /// A URL to the license of the image
    pub license: Option<String>,
}

impl ImageEntry {
    pub fn new(loc: impl Into<String>) -> Self {
        Self {
            loc: loc.into(),
            caption: None,
            geo_location: None,
            title: None,
            license: None,
        }
    }

    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn geo_location(mut self, geo: GeoLocation) -> Self {
        self.geo_location = Some(geo);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }
}

/// A URL entry with associated images
#[derive(Debug, Clone)]
pub struct UrlWithImages {
    pub url: UrlEntry,
    pub images: Vec<ImageEntry>,
}

impl UrlWithImages {
    pub fn new(url: UrlEntry) -> Self {
        Self {
            url,
            images: Vec::new(),
        }
    }

    pub fn add_image(mut self, image: ImageEntry) -> Self {
        self.images.push(image);
        self
    }

    pub fn add_images(mut self, images: Vec<ImageEntry>) -> Self {
        self.images.extend(images);
        self
    }
}

/// Video platform restriction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoPlatform {
    Web,
    Mobile,
    Tv,
}

impl VideoPlatform {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoPlatform::Web => "web",
            VideoPlatform::Mobile => "mobile",
            VideoPlatform::Tv => "tv",
        }
    }
}

/// Video platform restriction type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoPlatformRestriction {
    Allow(Vec<VideoPlatform>),
    Deny(Vec<VideoPlatform>),
}

/// Video country restriction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoCountryRestriction {
    Allow(Vec<String>),
    Deny(Vec<String>),
}

/// Video price
#[derive(Debug, Clone)]
pub struct VideoPrice {
    pub currency: String,
    pub amount: f32,
    pub resolution: Option<String>,
    pub type_: Option<String>,
}

impl VideoPrice {
    pub fn new(currency: impl Into<String>, amount: f32) -> Self {
        Self {
            currency: currency.into(),
            amount,
            resolution: None,
            type_: None,
        }
    }

    pub fn resolution(mut self, resolution: impl Into<String>) -> Self {
        self.resolution = Some(resolution.into());
        self
    }

    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }
}

/// Whether the video requires a subscription
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoRequiresSubscription {
    Yes,
    No,
}

impl VideoRequiresSubscription {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoRequiresSubscription::Yes => "yes",
            VideoRequiresSubscription::No => "no",
        }
    }
}

/// Video uploader information
#[derive(Debug, Clone)]
pub struct VideoUploader {
    pub name: String,
    pub info_url: Option<String>,
}

impl VideoUploader {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            info_url: None,
        }
    }

    pub fn info_url(mut self, url: impl Into<String>) -> Self {
        self.info_url = Some(url.into());
        self
    }
}

/// Whether the video is live
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoLive {
    Yes,
    No,
}

impl VideoLive {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoLive::Yes => "yes",
            VideoLive::No => "no",
        }
    }
}

/// A video entry in a video sitemap
#[derive(Debug, Clone)]
pub struct VideoEntry {
    /// URL of the thumbnail image (required)
    pub thumbnail_loc: String,

    /// Title of the video (required, max 100 characters)
    pub title: String,

    /// Description of the video (required, max 2048 characters)
    pub description: String,

    /// URL of the video content
    pub content_loc: Option<String>,

    /// URL of the video player
    pub player_loc: Option<String>,

    /// Duration of the video in seconds (0-28800)
    pub duration: Option<u32>,

    /// Publication date
    pub publication_date: Option<String>,

    /// Expiration date
    pub expiration_date: Option<String>,

    /// Rating of the video (0.0 to 5.0)
    pub rating: Option<f32>,

    /// Number of views
    pub view_count: Option<u64>,

    /// Whether the video is family friendly
    pub family_friendly: Option<bool>,

    /// Tags associated with the video
    pub tags: Vec<String>,

    /// Video category
    pub category: Option<String>,

    /// Country restrictions
    pub restriction: Option<VideoCountryRestriction>,

    /// Gallery title
    pub gallery_loc: Option<String>,

    /// Platform restrictions
    pub platform: Option<VideoPlatformRestriction>,

    /// Prices
    pub prices: Vec<VideoPrice>,

    /// Requires subscription
    pub requires_subscription: Option<VideoRequiresSubscription>,

    /// Uploader information
    pub uploader: Option<VideoUploader>,

    /// Whether the video is live
    pub live: Option<VideoLive>,
}

impl VideoEntry {
    pub fn new(
        thumbnail_loc: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            thumbnail_loc: thumbnail_loc.into(),
            title: title.into(),
            description: description.into(),
            content_loc: None,
            player_loc: None,
            duration: None,
            publication_date: None,
            expiration_date: None,
            rating: None,
            view_count: None,
            family_friendly: None,
            tags: Vec::new(),
            category: None,
            restriction: None,
            gallery_loc: None,
            platform: None,
            prices: Vec::new(),
            requires_subscription: None,
            uploader: None,
            live: None,
        }
    }

    pub fn content_loc(mut self, loc: impl Into<String>) -> Self {
        self.content_loc = Some(loc.into());
        self
    }

    pub fn player_loc(mut self, loc: impl Into<String>) -> Self {
        self.player_loc = Some(loc.into());
        self
    }

    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn publication_date(mut self, date: impl Into<String>) -> Self {
        self.publication_date = Some(date.into());
        self
    }

    pub fn expiration_date(mut self, date: impl Into<String>) -> Self {
        self.expiration_date = Some(date.into());
        self
    }

    pub fn rating(mut self, rating: f32) -> Self {
        self.rating = Some(rating);
        self
    }

    pub fn view_count(mut self, count: u64) -> Self {
        self.view_count = Some(count);
        self
    }

    pub fn family_friendly(mut self, family_friendly: bool) -> Self {
        self.family_friendly = Some(family_friendly);
        self
    }

    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn add_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn restriction(mut self, restriction: VideoCountryRestriction) -> Self {
        self.restriction = Some(restriction);
        self
    }

    pub fn gallery_loc(mut self, loc: impl Into<String>) -> Self {
        self.gallery_loc = Some(loc.into());
        self
    }

    pub fn platform(mut self, platform: VideoPlatformRestriction) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn add_price(mut self, price: VideoPrice) -> Self {
        self.prices.push(price);
        self
    }

    pub fn requires_subscription(mut self, requires: VideoRequiresSubscription) -> Self {
        self.requires_subscription = Some(requires);
        self
    }

    pub fn uploader(mut self, uploader: VideoUploader) -> Self {
        self.uploader = Some(uploader);
        self
    }

    pub fn live(mut self, live: VideoLive) -> Self {
        self.live = Some(live);
        self
    }
}

/// A URL entry with associated videos
#[derive(Debug, Clone)]
pub struct UrlWithVideos {
    pub url: UrlEntry,
    pub videos: Vec<VideoEntry>,
}

impl UrlWithVideos {
    pub fn new(url: UrlEntry) -> Self {
        Self {
            url,
            videos: Vec::new(),
        }
    }

    pub fn add_video(mut self, video: VideoEntry) -> Self {
        self.videos.push(video);
        self
    }

    pub fn add_videos(mut self, videos: Vec<VideoEntry>) -> Self {
        self.videos.extend(videos);
        self
    }
}

/// Entry for a sitemap index
#[derive(Debug, Clone)]
pub struct SitemapIndexEntry {
    /// URL of the sitemap
    pub loc: String,

    /// Last modification date
    pub lastmod: Option<String>,
}

impl SitemapIndexEntry {
    pub fn new(loc: impl Into<String>) -> Self {
        Self {
            loc: loc.into(),
            lastmod: None,
        }
    }

    pub fn lastmod(mut self, lastmod: impl Into<String>) -> Self {
        self.lastmod = Some(lastmod.into());
        self
    }
}

/// Publication information for a news article
#[derive(Debug, Clone)]
pub struct NewsPublication {
    /// Name of the news publication (required)
    /// Must exactly match the name as it appears on news.google.com
    pub name: String,

    /// Language code (required)
    /// ISO 639 standard (2-3 letters)
    /// Exception: zh-cn for Simplified Chinese, zh-tw for Traditional Chinese
    pub language: String,
}

impl NewsPublication {
    pub fn new(name: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            language: language.into(),
        }
    }
}

/// A news article entry for Google News sitemap
#[derive(Debug, Clone)]
pub struct NewsEntry {
    /// Publication information (required)
    pub publication: NewsPublication,

    /// Article publication date (required)
    /// W3C format: YYYY-MM-DD or YYYY-MM-DDThh:mm:ssTZD
    pub publication_date: String,

    /// Article headline (required)
    /// Should not include author name, publication name, or date
    pub title: String,

    /// Keywords describing the topic (optional)
    /// Comma-separated list
    pub keywords: Option<String>,

    /// Stock tickers related to the article (optional)
    /// Up to 5 tickers, comma-separated
    pub stock_tickers: Option<String>,
}

impl NewsEntry {
    pub fn new(
        publication: NewsPublication,
        publication_date: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            publication,
            publication_date: publication_date.into(),
            title: title.into(),
            keywords: None,
            stock_tickers: None,
        }
    }

    pub fn keywords(mut self, keywords: impl Into<String>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    pub fn stock_tickers(mut self, tickers: impl Into<String>) -> Self {
        self.stock_tickers = Some(tickers.into());
        self
    }
}

/// A URL entry with associated news article
#[derive(Debug, Clone)]
pub struct UrlWithNews {
    pub url: UrlEntry,
    pub news: NewsEntry,
}

impl UrlWithNews {
    pub fn new(url: UrlEntry, news: NewsEntry) -> Self {
        Self { url, news }
    }
}

/// A URL entry that can combine multiple sitemap extensions
/// Allows combining image, video, and news metadata in a single URL
#[derive(Debug, Clone)]
pub struct UrlWithExtensions {
    pub url: UrlEntry,
    pub images: Vec<ImageEntry>,
    pub videos: Vec<VideoEntry>,
    pub news: Option<NewsEntry>,
}

impl UrlWithExtensions {
    /// Create a new URL with extensions
    pub fn new(url: UrlEntry) -> Self {
        Self {
            url,
            images: Vec::new(),
            videos: Vec::new(),
            news: None,
        }
    }

    /// Add an image to this URL
    pub fn add_image(mut self, image: ImageEntry) -> Self {
        self.images.push(image);
        self
    }

    /// Add multiple images to this URL
    pub fn add_images(mut self, images: Vec<ImageEntry>) -> Self {
        self.images.extend(images);
        self
    }

    /// Add a video to this URL
    pub fn add_video(mut self, video: VideoEntry) -> Self {
        self.videos.push(video);
        self
    }

    /// Add multiple videos to this URL
    pub fn add_videos(mut self, videos: Vec<VideoEntry>) -> Self {
        self.videos.extend(videos);
        self
    }

    /// Set news metadata for this URL
    pub fn set_news(mut self, news: NewsEntry) -> Self {
        self.news = Some(news);
        self
    }

    /// Check if this URL has any extensions
    pub fn has_extensions(&self) -> bool {
        !self.images.is_empty() || !self.videos.is_empty() || self.news.is_some()
    }
}
