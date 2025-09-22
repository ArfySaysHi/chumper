use std::collections::HashMap;

pub fn group_by_key<T, F>(items: Vec<T>, mut get_key: F) -> HashMap<i64, Vec<T>>
where
    F: FnMut(&T) -> Option<i64>,
{
    let mut map: HashMap<i64, Vec<T>> = HashMap::new();
    for item in items {
        if let Some(key) = get_key(&item) {
            map.entry(key).or_default().push(item);
        }
    }

    map
}
