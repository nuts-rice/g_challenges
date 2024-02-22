#![no_main]
use libfuzzer_sys::fuzz_target;
// use client::api::*;
fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
});
