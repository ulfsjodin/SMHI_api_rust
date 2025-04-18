use std::{i64, result};

use omstart_smhi::json::timestamp_converter::{self, millis_to_cest};

#[test]
fn test_millis_to_cest_validation() {
    // 1 januari 2023, kl 00:00:00 UTC = 1672531200000 ms
    let millis = 1672531200000;
    let result = millis_to_cest(millis);
    assert_eq!(result, Some("23-01-01 01:00:00".to_string()));
}

fn test_millis_to_cest_outof_range() {
    // för stor (för i64) eller negativt nummer
    let millis = i64::MAX;
    let result = millis_to_cest(millis);
    assert!(result.is_none());
}