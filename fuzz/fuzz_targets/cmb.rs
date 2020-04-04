#![no_main]
use ccg2lambda_rs::lambda::cmb;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(mut s) = std::str::from_utf8(data) {
        let _ = cmb(&mut s);
    }
});
