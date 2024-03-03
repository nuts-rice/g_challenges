#![no_main]
#[macro_use] extern crate libfuzzer_sys;
// use client::api::*;
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
    }
    // fuzzed code goes here
});
