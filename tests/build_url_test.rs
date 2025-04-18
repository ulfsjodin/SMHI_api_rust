use omstart_smhi::{smhi::build_url, smhi::Parametrar};

#[test]
fn test_build_url() {
    let url = build_url(Parametrar::Temperatur, 12345);
    assert_eq!(
        url,
        "https://opendata-download-metobs.smhi.se/api/version/latest/parameter/1/station/12345/period/latest-day/data.json"
    );

}