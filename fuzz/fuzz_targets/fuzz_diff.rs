#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|input: (&str, &str)| {
    let (str1, str2) = input;
    let _ = dissimilar::diff(str1, str2);
});
