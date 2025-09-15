use super::PriorityBundle;
use std::collections::HashMap;

pub fn nest_priority_bundles(flat: Vec<PriorityBundle>) -> Vec<PriorityBundle> {
    log::debug!("nest_priority_bundles with {:#?}", &flat);
    let mut children_map: HashMap<i64, Vec<PriorityBundle>> = HashMap::new();
    let mut roots: Vec<PriorityBundle> = Vec::new();

    for pb in flat {
        match pb.parent_bundle_id {
            Some(pid) => {
                children_map.entry(pid).or_default().push(pb);
            }
            None => roots.push(pb),
        }
    }

    for pb in &mut roots {
        if let Some(pb_id) = pb.id {
            if let Some(children) = children_map.remove(&pb_id) {
                for child in children {
                    pb.children
                        .entry(child.grade.clone())
                        .or_default()
                        .push(child);
                }
            }
        }
    }

    roots
}

pub fn group_by_grade<T>(
    items: Vec<T>,
    grade_accessor: impl Fn(&T) -> &str,
) -> HashMap<String, Vec<T>> {
    let mut grouped: HashMap<String, Vec<T>> = HashMap::new();
    for item in items {
        let grade = grade_accessor(&item).to_string();
        grouped.entry(grade).or_default().push(item);
    }
    grouped
}
