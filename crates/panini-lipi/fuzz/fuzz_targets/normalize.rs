#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Must never panic on arbitrary valid UTF-8.
        let _ = panini_lipi::normalize(s);
    }
});
