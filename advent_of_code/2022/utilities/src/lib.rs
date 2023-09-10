
pub mod import {
    use std::fs;

    pub fn get_input(path: &str) -> String {
        let message = format!("Should be getting input from a file: {}", &path);
        let input = fs::read_to_string(path).expect(&message);
        input
    }

}