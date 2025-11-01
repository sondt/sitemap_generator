use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use sitemap_generator::{
    ChangeFreq, GeoLocation, ImageEntry, ImageSitemapBuilder, SitemapBuilder,
    SitemapIndexBuilder, SitemapIndexEntry, UrlEntry, UrlWithImages, UrlWithVideos, VideoEntry,
    VideoSitemapBuilder,
};

/// Benchmark building a standard sitemap with varying number of URLs
fn bench_standard_sitemap(c: &mut Criterion) {
    let mut group = c.benchmark_group("standard_sitemap");

    for size in [10, 100, 1_000, 10_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut builder = SitemapBuilder::new();
                for i in 0..size {
                    builder.add_url(
                        UrlEntry::new(format!("https://example.com/page/{}", i))
                            .lastmod("2025-11-01")
                            .changefreq(ChangeFreq::Daily)
                            .priority(0.8),
                    );
                }
                black_box(builder.build().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmark building sitemap as bytes
fn bench_build_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_bytes");

    for size in [10, 100, 1_000, 10_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut builder = SitemapBuilder::new();
                for i in 0..size {
                    builder.add_url(UrlEntry::new(format!("https://example.com/page/{}", i)));
                }
                black_box(builder.build_bytes().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmark building compressed sitemap
fn bench_build_compressed(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_compressed");

    for size in [10, 100, 1_000, 10_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut builder = SitemapBuilder::new();
                for i in 0..size {
                    builder.add_url(UrlEntry::new(format!("https://example.com/page/{}", i)));
                }
                black_box(builder.build_compressed_bytes().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmark image sitemap generation
fn bench_image_sitemap(c: &mut Criterion) {
    let mut group = c.benchmark_group("image_sitemap");

    for size in [10, 100, 1_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut builder = ImageSitemapBuilder::new();
                for i in 0..size {
                    let url_with_images = UrlWithImages::new(
                        UrlEntry::new(format!("https://example.com/gallery/{}", i))
                            .lastmod("2025-11-01")
                            .priority(0.9),
                    )
                    .add_image(
                        ImageEntry::new(format!("https://example.com/images/photo{}.jpg", i))
                            .title(format!("Photo {}", i))
                            .caption("A beautiful photo")
                            .geo_location(
                                GeoLocation::new()
                                    .city("San Francisco")
                                    .state("CA")
                                    .country("USA"),
                            ),
                    )
                    .add_image(
                        ImageEntry::new(format!("https://example.com/images/photo{}_2.jpg", i))
                            .title(format!("Photo {} - Alt", i))
                            .caption("Another beautiful photo"),
                    );

                    builder.add_url(url_with_images);
                }
                black_box(builder.build().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmark video sitemap generation
fn bench_video_sitemap(c: &mut Criterion) {
    let mut group = c.benchmark_group("video_sitemap");

    for size in [10, 100, 1_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut builder = VideoSitemapBuilder::new();
                for i in 0..size {
                    let url_with_video = UrlWithVideos::new(
                        UrlEntry::new(format!("https://example.com/videos/{}", i))
                            .lastmod("2025-11-01")
                            .priority(0.9),
                    )
                    .add_video(
                        VideoEntry::new(
                            format!("https://example.com/thumbnails/video{}.jpg", i),
                            format!("Video {}", i),
                            format!("Description for video {}", i),
                        )
                        .content_loc(format!("https://example.com/videos/video{}.mp4", i))
                        .duration(300)
                        .publication_date("2025-11-01")
                        .rating(4.5)
                        .view_count(10000)
                        .family_friendly(true)
                        .add_tag("tutorial")
                        .add_tag("product")
                        .category("Education"),
                    );

                    builder.add_url(url_with_video);
                }
                black_box(builder.build().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmark sitemap index generation
fn bench_sitemap_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("sitemap_index");

    for size in [10, 100, 1_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut builder = SitemapIndexBuilder::new();
                for i in 0..size {
                    builder.add_sitemap(
                        SitemapIndexEntry::new(format!(
                            "https://example.com/sitemap{}.xml.gz",
                            i
                        ))
                        .lastmod("2025-11-01"),
                    );
                }
                black_box(builder.build().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmark compression ratio
fn bench_compression_ratio(c: &mut Criterion) {
    let group = c.benchmark_group("compression_ratio");

    // Build a sitemap once for comparison
    let sizes = vec![100, 1_000, 10_000];

    for size in sizes {
        let mut builder = SitemapBuilder::new();
        for i in 0..size {
            builder.add_url(
                UrlEntry::new(format!("https://example.com/page/{}", i))
                    .lastmod("2025-11-01")
                    .changefreq(ChangeFreq::Daily)
                    .priority(0.8),
            );
        }

        let uncompressed = builder.build_bytes().unwrap();
        let compressed = builder.build_compressed_bytes().unwrap();

        let ratio = uncompressed.len() as f64 / compressed.len() as f64;

        eprintln!(
            "{} URLs: {} bytes -> {} bytes ({}x compression)",
            size,
            uncompressed.len(),
            compressed.len(),
            ratio
        );
    }

    group.finish();
}

/// Benchmark validation overhead
fn bench_validation_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation_overhead");

    let size = 1_000;

    // With validation
    group.bench_function("with_validation", |b| {
        b.iter(|| {
            let mut builder = SitemapBuilder::new();
            for i in 0..size {
                builder.add_url(
                    UrlEntry::new(format!("https://example.com/page/{}", i))
                        .lastmod("2025-11-01")
                        .priority(0.8),
                );
            }
            black_box(builder.build().unwrap());
        });
    });

    // Without validation
    group.bench_function("without_validation", |b| {
        b.iter(|| {
            let mut builder = SitemapBuilder::new().validate(false);
            for i in 0..size {
                builder.add_url(
                    UrlEntry::new(format!("https://example.com/page/{}", i))
                        .lastmod("2025-11-01")
                        .priority(0.8),
                );
            }
            black_box(builder.build().unwrap());
        });
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    // Pre-allocated vs dynamic
    group.bench_function("dynamic_allocation", |b| {
        b.iter(|| {
            let mut builder = SitemapBuilder::new();
            for i in 0..1000 {
                builder.add_url(UrlEntry::new(format!("https://example.com/page/{}", i)));
            }
            black_box(builder.build().unwrap());
        });
    });

    group.bench_function("pre_allocated", |b| {
        b.iter(|| {
            let mut urls = Vec::with_capacity(1000);
            for i in 0..1000 {
                urls.push(UrlEntry::new(format!("https://example.com/page/{}", i)));
            }
            let mut builder = SitemapBuilder::new();
            builder.add_urls(urls);
            black_box(builder.build().unwrap());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_standard_sitemap,
    bench_build_bytes,
    bench_build_compressed,
    bench_image_sitemap,
    bench_video_sitemap,
    bench_sitemap_index,
    bench_compression_ratio,
    bench_validation_overhead,
    bench_memory_allocation,
);

criterion_main!(benches);
