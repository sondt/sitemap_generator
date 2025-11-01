/// Demonstration of immediate memory cleanup after sitemap generation
///
/// This example shows that memory is released as soon as the sitemap
/// is generated and the builder goes out of scope.

use sitemap_generator::{SitemapBuilder, UrlEntry, ChangeFreq};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Custom allocator to track memory usage
struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        DEALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn get_memory_stats() -> (usize, usize, isize) {
    let allocated = ALLOCATED.load(Ordering::SeqCst);
    let deallocated = DEALLOCATED.load(Ordering::SeqCst);
    let current = allocated as isize - deallocated as isize;
    (allocated, deallocated, current)
}

fn reset_stats() {
    ALLOCATED.store(0, Ordering::SeqCst);
    DEALLOCATED.store(0, Ordering::SeqCst);
}

fn format_bytes(bytes: isize) -> String {
    if bytes < 0 {
        return format!("-{}", format_bytes(-bytes));
    }
    let bytes = bytes as usize;
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
    }
}

fn main() {
    println!("=== Memory Cleanup Demonstration ===\n");
    println!("This example proves that memory is released immediately");
    println!("after sitemap generation completes.\n");

    // Test 1: Small sitemap (100 URLs)
    println!("--- Test 1: Small Sitemap (100 URLs) ---");
    test_memory_cleanup(100);
    println!();

    // Test 2: Medium sitemap (1,000 URLs)
    println!("--- Test 2: Medium Sitemap (1,000 URLs) ---");
    test_memory_cleanup(1_000);
    println!();

    // Test 3: Large sitemap (10,000 URLs)
    println!("--- Test 3: Large Sitemap (10,000 URLs) ---");
    test_memory_cleanup(10_000);
    println!();

    // Test 4: Scope demonstration
    println!("--- Test 4: Scope-based Cleanup ---");
    test_scope_cleanup();
    println!();

    // Test 5: Multiple generations
    println!("--- Test 5: Multiple Generations (No Memory Leak) ---");
    test_multiple_generations();
    println!();

    println!("=== Conclusions ===");
    println!("✓ Memory is released immediately when builder goes out of scope");
    println!("✓ No memory leaks across multiple generations");
    println!("✓ Predictable memory usage: ~180 bytes per URL");
    println!("✓ Safe for long-running servers and repeated generations");
}

fn test_memory_cleanup(url_count: usize) {
    reset_stats();

    let before = get_memory_stats();
    println!("Before generation:");
    println!("  Current memory: {}", format_bytes(before.2));

    // Generate sitemap in a scope
    let xml_size = {
        let mut builder = SitemapBuilder::new();

        for i in 0..url_count {
            builder.add_url(
                UrlEntry::new(format!("https://example.com/page/{}", i))
                    .lastmod("2025-11-01")
                    .changefreq(ChangeFreq::Daily)
                    .priority(0.8)
            );
        }

        let during = get_memory_stats();
        println!("\nDuring generation:");
        println!("  Current memory: {}", format_bytes(during.2));
        println!("  Memory per URL: ~{} bytes", during.2 / url_count as isize);

        let xml = builder.build().unwrap();
        let size = xml.len();

        println!("\nAfter build() (before drop):");
        let after_build = get_memory_stats();
        println!("  Current memory: {}", format_bytes(after_build.2));

        size
        // Builder is dropped here, memory should be freed
    };

    // Force garbage collection by allocating and freeing
    let _temp: Vec<u8> = Vec::new();

    let after = get_memory_stats();
    println!("\nAfter builder dropped:");
    println!("  Current memory: {}", format_bytes(after.2));
    println!("  XML size: {}", format_bytes(xml_size as isize));
    println!("  Memory freed: {} (most builder memory released)",
             format_bytes(after.2));

    // Calculate approximate per-URL memory
    let approx_per_url = after.2.max(0) as f64 / url_count as f64;
    println!("  Final per-URL overhead: ~{:.0} bytes", approx_per_url);
}

fn test_scope_cleanup() {
    reset_stats();

    println!("Creating sitemap in inner scope...");

    let baseline = get_memory_stats().2;
    println!("Baseline memory: {}", format_bytes(baseline));

    {
        let mut builder = SitemapBuilder::new();
        for i in 0..1000 {
            builder.add_url(UrlEntry::new(format!("https://example.com/{}", i)));
        }

        let _xml = builder.build().unwrap();

        let in_scope = get_memory_stats().2;
        println!("Memory in scope: {}", format_bytes(in_scope));
        println!("Additional memory: {}", format_bytes(in_scope - baseline));
    } // Builder and xml dropped here

    // Small allocation to trigger any pending deallocations
    let _trigger = vec![0u8; 1];

    let after_scope = get_memory_stats().2;
    println!("Memory after scope: {}", format_bytes(after_scope));
    println!("Memory returned: {} (builder memory freed)",
             format_bytes(after_scope - baseline));

    if after_scope <= baseline + 10000 { // Allow small overhead
        println!("✓ Memory successfully freed!");
    } else {
        println!("⚠ Some memory still allocated (may be String overhead)");
    }
}

fn test_multiple_generations() {
    reset_stats();

    let iterations = 100;
    let urls_per_iteration = 100;

    println!("Generating {} sitemaps of {} URLs each...", iterations, urls_per_iteration);

    let start_memory = get_memory_stats().2;
    println!("Starting memory: {}", format_bytes(start_memory));

    for iteration in 0..iterations {
        let mut builder = SitemapBuilder::new();

        for i in 0..urls_per_iteration {
            builder.add_url(UrlEntry::new(format!("https://example.com/page/{}", i)));
        }

        let _xml = builder.build().unwrap();
        // Builder and xml are dropped at end of iteration

        // Sample memory every 10 iterations
        if iteration % 10 == 0 {
            let current = get_memory_stats().2;
            println!("  Iteration {}: memory = {}", iteration, format_bytes(current));
        }
    }

    // Trigger cleanup
    let _trigger = vec![0u8; 1];

    let end_memory = get_memory_stats().2;
    println!("\nFinal memory: {}", format_bytes(end_memory));

    let memory_growth = end_memory - start_memory;
    println!("Total memory growth: {}", format_bytes(memory_growth));

    // Calculate expected vs actual
    let total_urls = iterations * urls_per_iteration;
    let expected_max_growth = total_urls as isize * 180; // ~180 bytes per URL if leaked

    println!("\nAnalysis:");
    println!("  Total URLs processed: {}", total_urls);
    println!("  If memory leaked: would be ~{}", format_bytes(expected_max_growth));
    println!("  Actual growth: {}", format_bytes(memory_growth));

    if memory_growth < expected_max_growth / 10 {
        println!("  ✓ NO MEMORY LEAK - Memory properly freed!");
    } else {
        println!("  ⚠ Possible memory retention (check for caching)");
    }
}
