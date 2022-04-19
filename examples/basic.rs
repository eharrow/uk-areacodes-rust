use uk_areacodes::api::binary_search;
use uk_areacodes::api::find_by_code;
use uk_areacodes::api::starts_with_code;
use uk_areacodes::api::Place;
use uk_areacodes::data_source::json::UK;

fn main() {
    let data: Vec<Place> = serde_json::from_str(&UK).expect("JSON was not well-formatted");

    if let Some(p) = find_by_code("01727", &data) {
        println!("area is {:#?}", p.area);
    }

    if let Some(p) = starts_with_code("01328", &data) {
        println!("{:#?}", p)
    }

    if let Some(n) = binary_search(&data, 0, data.len() - 1, "01503") {
        println!("Found place named {}", data[n as usize].area)
    }
}
