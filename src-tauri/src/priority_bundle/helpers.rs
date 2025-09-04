use super::PriorityBundle;
use std::collections::HashMap;

pub fn nest_priority_bundles(flat: Vec<PriorityBundle>) -> Vec<PriorityBundle> {
    let mut by_id: HashMap<i64, PriorityBundle> = HashMap::new();
    let mut no_id_items: Vec<PriorityBundle> = Vec::new();

    for mut item in flat {
        item.priority_bundles = Vec::new();

        match item.id {
            Some(id) => {
                by_id.insert(id, item);
            }
            None => {
                no_id_items.push(item);
            }
        }
    }

    let ids: Vec<i64> = by_id.keys().copied().collect();
    for id in ids {
        if let Some(child) = by_id.remove(&id) {
            match child.parent_bundle_id {
                Some(parent_id) => {
                    if let Some(parent) = by_id.get_mut(&parent_id) {
                        parent.priority_bundles.push(child);
                    } else {
                        by_id.insert(id, child);
                    }
                }
                None => {
                    by_id.insert(id, child);
                }
            }
        }
    }

    let mut roots: Vec<PriorityBundle> = by_id
        .into_values()
        .filter(|pb| pb.parent_bundle_id.is_none())
        .collect();
    roots.extend(no_id_items.into_iter());

    roots
}
