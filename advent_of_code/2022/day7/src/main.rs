use std::collections::HashMap;
use std::path::PathBuf;
use utilities::import;

fn main() {
    let base_case_input = import::get_input("inputs/input0.txt");
    let base_case_sum_of_directory_size = part_1_calculation(&base_case_input);
    println!("base_case_sum_of_directory_size={}", base_case_sum_of_directory_size);

    let part_1_input = import::get_input("inputs/input1.txt");
    let part_1_sum_of_directory_size = part_1_calculation(&part_1_input);
    println!("part_1_sum_of_directory_size={}", part_1_sum_of_directory_size);
    let part_2_sum_of_directory_size = part_2_calculation(&part_1_input);
    println!("part_2_sum_of_directory_size={}", part_2_sum_of_directory_size);
}
fn part_1_calculation(input: &str) -> usize  {
    let size_limit: usize = 100000;
    let input_lines = input.split("\n").collect::<Vec<&str>>();
    let file_system_map = build_file_system_map(input_lines);

    let sum_of_directory_size: usize = file_system_map
        .into_values()
        .filter(|size| {
            *size <= size_limit
        }).sum();
    return sum_of_directory_size
}

fn part_2_calculation(input: &str) -> usize {
    let input_lines = input.split("\n").collect::<Vec<&str>>();
    let path_to_size_map = build_file_system_map(input_lines);

    let total_disk_space: usize = 70_000_000;
    let needed_space: usize = 30_000_000;
    let root_path = PathBuf::from("/");
    let current_root_size = path_to_size_map.get(&root_path).unwrap();
    let available = total_disk_space - current_root_size;

    let smallest_directory_size_that_satisfies: usize = path_to_size_map
        .into_values()
        .filter(|size|{
            size + available >= needed_space
        })
        .min()
        .unwrap();
    return smallest_directory_size_that_satisfies
}

fn build_file_system_map(input_lines: Vec<&str>) -> HashMap<PathBuf, usize> {
    let initial_directory_name_to_size_map: HashMap<PathBuf, usize> = HashMap::new();
    let initial_affected_directory_names: Vec<&str> = Vec::new();
    let result: (HashMap<PathBuf, usize>, Vec<&str>) = input_lines
        .iter()
        .fold((initial_directory_name_to_size_map, initial_affected_directory_names), |current_result, line|{
            let mut path_to_size_map: HashMap<PathBuf, usize> = current_result.0;
            let mut affected_directories: Vec<&str> = current_result.1;
            let line_parts: Vec<_> = line.split_whitespace().collect();

            if line.starts_with("$ ls") || line.starts_with("dir") {
                return (path_to_size_map, affected_directories)
            }
            match line_parts[..] {
                ["$", "cd", ".."] => {
                    affected_directories.pop();
                    return (path_to_size_map, affected_directories)
                }
                ["$", "cd", dir_name] => {
                    affected_directories.push(dir_name);
                    return (path_to_size_map, affected_directories)
                }
                [size, _name] => {
                    let size: usize = size.parse().unwrap();
                    let longest_path = affected_directories.len();
                    for index in 0..longest_path {
                        let path_parts = &affected_directories[..=index];
                        let path = PathBuf::from_iter(&*path_parts);
                        *path_to_size_map.entry(path).or_insert(0) += size;
                    }
                    return (path_to_size_map, affected_directories)
                }
                _ => {
                    return (path_to_size_map, affected_directories)
                }
            }
        });
    return result.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_base_case() {
        let input = import::get_input("inputs/input0.txt");
        let actual = part_1_calculation(&input);
        let expected: usize = 95437;
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_case() {
        let input = import::get_input("inputs/input1.txt");
        let actual = part_1_calculation(&input);
        let expected: usize = 1989474;
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_2_case() {
        let input = import::get_input("inputs/input1.txt");
        let actual = part_2_calculation(&input);
        let expected: usize = 1111607;
        assert_eq!(actual, expected);
    }

}