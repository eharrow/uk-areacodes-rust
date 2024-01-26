use std::collections::HashMap;
use uk_areacodes::api;
use uk_areacodes::data_source::json::{UK_AS_MAP, UK_AS_SEQ};

//noinspection SpellCheckingInspection
#[test]
fn it_returns_matches() {
    let data: Vec<api::Place> =
        serde_json::from_str(UK_AS_SEQ).expect("JSON was not well-formatted");

    if let Some(p) = api::starts_with_code("01328", &data) {
        assert_eq!("Fakenham", p.area);
    }

    if let Some(n) = api::binary_search(&data, 0, data.len() - 1, "01503") {
        assert_eq!("Looe", data[n].area);
    }
}

#[test]
fn it_returns_matches_with_map() {
    let data: HashMap<String, api::Place> =
        serde_json::from_str(UK_AS_MAP).expect("JSON was not well-formatted");

    if let Some(p) = api::find_by_code("01727", &data) {
        assert_eq!("St Albans", p.area);
    }
}
