use std::fs;

pub fn read_file(path: &str) -> String {
        assert_ne!(path.len(), 0, "Path to file is invalid!");
        let content: String = fs::read_to_string(path).expect(&format!("file {path} not found"));
        return content;
    }