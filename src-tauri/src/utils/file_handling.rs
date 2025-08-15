use std::string::String;
use walkdir::WalkDir;

pub fn get_ext_from(path: String, ext: String) -> Vec<String> {
    let walker = WalkDir::new(path).into_iter().filter_map(|e| e.ok());
    let mut file_paths = vec![];

    for entry in walker {
        let entry_path = entry
            .path()
            .to_str()
            .expect("file_handling: Failed to convert file path into a string.");

        if entry_path.ends_with(&ext) {
            file_paths.push(entry_path.to_owned());
        }
    }

    file_paths
}
