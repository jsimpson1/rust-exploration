use std::collections::HashMap;
use utilities::import;

const PATH: &str = "inputs/input0.txt";

fn main() {
    let input = import::get_input(PATH);
    println!("inputs=\n{}", input);
    let size_limit: usize = 100000;
    let sum_of_directory_size: usize = sum_directories_smaller_than_limit(size_limit, &input);
    println!("sum_of_directory_sizes={}", sum_of_directory_size)
}

fn sum_directories_smaller_than_limit(size_limit: usize, input: &str) -> usize {
    let input_lines = input.split("\n").collect::<Vec<&str>>();
    let files_system_map = build_file_system_map(input_lines);
    let answer = calculate_sum_of_size_to_cleanup(file_system_map, size_limit);
    return answer
}

trait Node {
    fn total_size(&self) -> usize;
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize
}

impl Node for File {
    fn total_size(&self) -> usize {
        return self.size;
    }
}

#[derive(Debug)]
struct Directory {
    parent: Option<Directory>,
    children: Option<Vec<dyn Node>>
}

impl Node for Directory {
    fn total_size(&self) -> usize {
        self.children.map(|c| {
            c match {
                File(name, size) => size;
            }
        });
        return 0
    }
}

fn build_file_system_map(input_lines: Vec<&str>) {
    let v = input_lines.iter().map(|&line| {
        let line_components = line.split_whitespace().collect::Vec<_>();
    });
}

fn calculate_sum_of_size_to_cleanup(file_system_map: &str, size_limit: usize) -> usize {
    return -1
}