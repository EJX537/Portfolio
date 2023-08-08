use std::collections::HashMap;

pub fn to_string<'a>(hmap: &'a HashMap<&'a str, &'a str>, key: &'a str) -> Option<&'a str> {
    if let Some(value) = hmap.get(key) {
        Some(*value)
    } else {
        None
    }
}
