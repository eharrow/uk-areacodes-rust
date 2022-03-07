use std::env;
use uk_areacodes::api::Place;
use uk_areacodes::api::{binary_search, find_by_code, starts_with_code};
use uk_areacodes::data_source::json::UK;

fn print_usage() {
    eprintln!("Usage: uk-areacodes <code>");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        print_usage();
        eprintln!("wrong number of arguments: expected 1, got {}.", args.len());
        // eprintln!("{} wrong number of arguments: expected 4, got {}.",
        //     "Error:".red().bold(), args.len());
        std::process::exit(1);
    }
    let num = &args[0];

    let data: Vec<Place> = serde_json::from_str(&UK).expect("JSON was not well-formatted");

    // if let Some(p) = find_by_code("01727", &data) {
    //     println!("area is {:#?}", p.area);
    // }

    // if let Some(p) = starts_with_code("01328", &data) {
    //     println!("{:#?}", p)
    // }

    if let Some(n) = binary_search(&data, 0, data.len() - 1, num) {
        println!("{}", data[n as usize].area)
    }
}
