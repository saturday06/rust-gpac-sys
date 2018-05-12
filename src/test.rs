use super::*;
use std::str;

#[test]
fn test() {
    println!(
        "GPAC_CONFIGURATION: {:?}",
        str::from_utf8(GPAC_CONFIGURATION)
    );
}
