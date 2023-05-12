#![no_main]

use dissimilar::Chunk;
use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|input: (&str, &str)| {
    // get chunks
    let (mut str1, mut str2) = input;
    let chunks = dissimilar::diff(str1, str2);
    
    // handle chunks
    for chunk in chunks {
        match chunk {
            Chunk::Equal(chunk) => {
                str1 = str1.strip_prefix(chunk).unwrap();
                str2 = str2.strip_prefix(chunk).unwrap();
            }
            Chunk::Delete(chunk) => {
                str1 = str1.strip_prefix(chunk).unwrap();
            }
            Chunk::Insert(chunk) => {
                str2 = str2.strip_prefix(chunk).unwrap();
            }
        }
    }

    // validate chunks
    assert!(str1.is_empty());
    assert!(str2.is_empty());
});
