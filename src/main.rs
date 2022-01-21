
mod data_source;
use crate::data_source::api::uk::UK_JSON;
mod lib;

fn main() {
    let data: Vec<lib::api::Place> =
        serde_json::from_str(&UK_JSON).expect("JSON was not well-formatted");

    if let Some(p) = lib::api::find_by_code("01727", &data) {
        println!("{:#?}", p)
    }

    if let Some(p) = lib::api::starts_with_code("01328", &data) {
        println!("{:#?}", p)
    }

    if let Some(n) = lib::api::binary_search(&data, 0, data.len() - 1, "01503") {
        println!("Found {}", data[n as usize].area)
    }
}
