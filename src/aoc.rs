use std::fs;

pub fn load_input(filepath: &str) -> String {
    fs::read_to_string(filepath).expect(format!("Failed to load file at {}", filepath).as_str())
}
