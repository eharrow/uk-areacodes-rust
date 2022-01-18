use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Place {
    code: String,
    area: String,
    ofcom_desc: String,
    previous_b_t_area_name: String,
}

fn main() {
    let file = File::open("areacodes-uk.json").unwrap();
    let data: Vec<Place> = serde_json::from_reader(file).expect("JSON was not well-formatted");
    let r = match find_by_code("01328", &data) {
        None => (),
        Some(p) => println!("{:#?}", p),
    };

    let s = match starts_with_code("0132", &data) {
        None => (),
        Some(p) => println!("{:#?}", p),
    };
}

fn find_by_code<'a>(prefix: &str, values: &'a Vec<Place>) -> Option<&'a Place> {
    for (item) in values.iter() {
        // println!("area: {} code: {}", item.area, item.code);
        if item.code == prefix {
            println!("match {}: {}", prefix, item.area);
            return Some(item);
        }
    }
    None
}

fn starts_with_code<'a>(number: &str, values: &'a Vec<Place>) -> Option<&'a Place> {
    for (item) in values.iter() {
        if item.code.starts_with(number)  {
            println!("match {}: {}", number, item.area);
            return Some(item);
        }
    }
    None
}