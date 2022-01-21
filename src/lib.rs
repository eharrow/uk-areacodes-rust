pub mod api {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Place {
        code: String,
        pub area: String,
        ofcom_desc: String,
        previous_b_t_area_name: String,
    }
    pub fn find_by_code<'a>(prefix: &str, values: &'a Vec<Place>) -> Option<&'a Place> {
        for item in values.iter() {
            // println!("area: {} code: {}", item.area, item.code);
            if item.code == prefix {
                // println!("match {}: {}", prefix, item.area);
                return Some(item);
            }
        }
        None
    }
    pub fn starts_with_code<'a>(number: &str, values: &'a Vec<Place>) -> Option<&'a Place> {
        for item in values.iter() {
            if number.starts_with(&item.code) {
                // println!("match {}: {}", number, item.area);
                return Some(item);
            }
        }
        None
    }
    pub fn binary_search(arr: &Vec<Place>, left: usize, right: usize, x: &str) -> Option<usize> {
        // dbg!(left, right, x);
        if left <= right && right >= 1 {
            let mid = left + (right - left) / 2;
            // println!("mid:{} - arr[mid]:{:#?}", mid, arr[mid].code);
            // If the element is present at the
            // middle itself
            if arr[mid].code == x {
                return Some(mid);
            }
            // If element is smaller than mid, then
            // it can only be present in left subarray
            if arr[mid].code > x.to_string() {
                // dbg!("element is smaller than mid so must be in left");
                return binary_search(arr, left, mid - 1, x);
            }
            // dbg!("element is larger than mid so must be in right");
            return binary_search(arr, mid + 1, right, x);
        }
        None
    }
}