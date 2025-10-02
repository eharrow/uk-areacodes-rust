pub mod data_source;

//noinspection SpellCheckingInspection
/// uk-areacodes api module for looking up a UK OFCOM area name from a phone number (land line).
///
/// You might use this give you an idea of where a caller is being made from assumming the number is not being spoofed.
pub mod api {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    //noinspection SpellCheckingInspection
    /// An OFCOM place
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Place {
        /// OFCOM area code
        code: String,
        /// The place's area
        pub area: String,
        /// OFCOM's description of the place
        ofcom_desc: String,
        /// OFCOM's previous name of the area
        previous_b_t_area_name: String,
        /// Geocoded latitude
        pub lat: Option<String>,
        /// Geocoded longitude
        pub lon: Option<String>
    }

    /// Initialises by loading the data and returning it as a list of Places
    /// ## Panics
    ///
    /// Will panic if the JSON it loads is not well-formed
    #[must_use]
    pub fn load() -> Vec<Place> {
        serde_json::from_str(crate::data_source::json::UK_AS_SEQ)
            .expect("JSON was not well-formatted")
    }

    /// Initialises by loading the data and returning it as a hashmap of place names to Places
    /// ## Panics
    ///
    /// Will panic if the JSON it loads is not well-formed
    #[must_use]
    pub fn load_as_map() -> HashMap<String, Place> {
        serde_json::from_str(crate::data_source::json::UK_AS_MAP)
            .expect("JSON was not well-formatted")
    }

    /// Finds a place by code prefix or STD as it is known in the UK
    #[allow(clippy::implicit_hasher)]
    #[must_use]
    pub fn find_by_code<'a>(prefix: &str, values: &'a HashMap<String, Place>) -> Option<&'a Place> {
        values.get(prefix)
    }

    /// Finds a place by code prefix or STD as it is known in the UK
    #[must_use]
    pub fn starts_with_code<'a>(number: &str, values: &'a [Place]) -> Option<&'a Place> {
        values.iter().find(|&item| number.starts_with(&item.code))
    }

    /// Finds a place by code prefix or STD as it is known in the UK.  A more efficient search method.
    #[must_use]
    pub fn binary_search(arr: &[Place], left: usize, right: usize, number: &str) -> Option<usize> {
        // dbg!(left, right, x);
        if left <= right && right >= 1 {
            let mid = left + (right - left) / 2;
            if arr[mid].code == number {
                return Some(mid);
            }
            // If element is smaller than mid, then
            // it can only be present in left subarray
            if *number < *arr[mid].code {
                // dbg!("element is smaller than mid so must be in left");
                return binary_search(arr, left, mid - 1, number);
            }
            // dbg!("element is larger than mid so must be in right");
            return binary_search(arr, mid + 1, right, number);
        }
        None
    }

    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn load_as_map_test() {
            let as_map = load_as_map();
            assert_eq!(as_map.len(), 611);
        }

        //noinspection SpellCheckingInspection
        #[test]
        fn find_by_code_test() {
            let data = load_as_map();

            if let Some(p) = find_by_code("01727", &data) {
                assert_eq!(p.area, "St Albans");
                assert_eq!(p.lat.as_ref().expect("oh dear"), "51.753051");
            }
        }

        //noinspection SpellCheckingInspection
        #[test]
        fn starts_with_code_test() {
            let data: Vec<Place> = serde_json::from_str(crate::data_source::json::UK_AS_SEQ)
                .expect("JSON was not well-formatted");
            if let Some(p) = starts_with_code("01328", &data) {
                assert_eq!(p.area, "Fakenham");
            }
        }

        //noinspection SpellCheckingInspection
        #[test]
        fn binary_search_test() {
            let data: Vec<Place> = serde_json::from_str(crate::data_source::json::UK_AS_SEQ)
                .expect("JSON was not well-formatted");

            if let Some(n) = binary_search(&data, 0, data.len() - 1, "01503") {
                assert_eq!(data[n].area, "Looe");
            }
        }
    }
}
