
use omstart_smhi::json::tempvalue_parser::parse_tempvalue;
use serde::Deserialize;

#[cfg(test)]
mod tests {
    use super::*; // Detta importerar allt från den överordnade modulen.

#[derive(Debug, Deserialize)]
struct TestStruct {
    #[serde(default, deserialize_with = "parse_tempvalue")]
    // #[serde(deserialize_with = "parse_tempvalue")]
    value: Option<f64>,
}

#[test]
fn test_parse_tempvalue_variants() {
    let cases = vec![
        (r#"{"value": "3.7"}"#, Some(3.7)),
        (r#"{"value": "NaN"}"#, None),
        (r#"{"value": "   "}"#, None),
        (r#"{"value": ""}"#, None),
        (r#"{"value": null}"#, None),
        (r#"{"value": "0"}"#, Some(0.0)),
        (r#"{}"#, None), // Här hanterar vi när "value" saknas
    ];

    for (json, expected) in cases {
        let result: Result<TestStruct, _> = serde_json::from_str(json);
        match result {
            Ok(res) => {
                println!("Testing: {}", json); 
                println!("Result: {:?}", res.value); 
                assert_eq!(res.value, expected, "Failed on input: {}", json);
            }
            Err(e) => {
                // Här kan vi fånga eventuella deserialiseringfel och skriva ut dem
                eprintln!("Error deserializing JSON: {}", e);
                assert_eq!(expected, None, "Failed on input: {}", json);
            }
        }
    }
}

#[test]
fn test_dummy() {
    println!("Test is running!");
    assert_eq!(2 + 2, 4);
}
}