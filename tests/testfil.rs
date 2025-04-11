use serde::Deserialize;
use serde::de::{self, Deserializer};
use std::str::FromStr;

fn parse_tempvalue<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(text) => {
            if text.trim().is_empty() || text == "NaN" {
                Ok(None)
            } else {
                f64::from_str(&text)
                    .map(Some)
                    .map_err(de::Error::custom)
            }
        }
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
struct TestStruct {
    #[serde(deserialize_with = "parse_tempvalue")]
    value: Option<f64>,
}

#[test]
fn test_parse_tempvalue_with_debug_output() {
    let cases = vec![
        (r#"{"value": "3.5"}"#, Some(3.5)),
        (r#"{"value": "1.0"}"#, Some(1.0)),
        (r#"{"value": "NaN"}"#, None),
        (r#"{"value": ""}"#, None),
        (r#"{"value": null}"#, None),
        (r#"{}"#, None),
    ];

    for (json, expected) in cases {
        println!("\n🔍 Testar JSON: {}", json);
        let result: TestStruct = serde_json::from_str(json).unwrap();
        println!("📦 Deserialiserat till: {:?}", result.value);
        println!("✅ Förväntat resultat: {:?}", expected);
        assert_eq!(result.value, expected);
    }
}

#[test]
fn test_invalid_value_as_result() {
    let result: Result<TestStruct, _> = serde_json::from_str(r#"{"value": oops}"#);
    assert!(result.is_err());
}
