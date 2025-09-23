use std::collections::HashMap;
use std::hash::Hash;

pub fn group_by_key<T, F, K>(items: Vec<T>, mut get_key: F) -> HashMap<K, Vec<T>>
where
    F: FnMut(&T) -> Option<K>,
    K: Eq + Hash,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        if let Some(key) = get_key(&item) {
            map.entry(key).or_default().push(item);
        }
    }
    map
}
