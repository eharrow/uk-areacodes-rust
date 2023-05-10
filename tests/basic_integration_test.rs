use uk_areacodes::api;
use uk_areacodes::data_source::json::UK;

#[test]
fn it_returns_matches() {
    let data: Vec<api::Place> = serde_json::from_str(&UK).expect("JSON was not well-formatted");

    if let Some(p) = api::find_by_code("01727", &data) {
        assert_eq!("St Albans", p.area)
    }

    if let Some(p) = api::starts_with_code("01328", &data) {
        assert_eq!("Fakenham", p.area);
    }

    if let Some(n) = api::binary_search(&data, 0, data.len() - 1, "01503") {
        assert_eq!("Looe", data[n as usize].area);

    }
}
