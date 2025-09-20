pub fn timestamp() -> Option<String> {
    Some("datetime('now')".to_string())
}

pub fn system() -> String {
    "Core".to_string()
}

pub fn priority() -> i64 {
    100
}

pub fn attribute_min() -> i32 {
    1
}

pub fn attribute_max() -> i32 {
    6
}

pub fn rating() -> i32 {
    1
}
