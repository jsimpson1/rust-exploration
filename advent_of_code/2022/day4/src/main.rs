use std::fs;

const PUZZLE_PART: u32 = 1;

fn main() {
    let input = get_input(PUZZLE_PART);
    println!("input:\n{}", input);
    let overlap_pairs = get_overlap_pairs(input);
    println!("overlap_pairs={}", overlap_pairs);
}

fn get_input(part: u32) -> String {
    let input_file_name = format!("src/input_part{}.txt", part);
    let message = format!("Should be getting input from a file: {}", input_file_name);
    let input = fs::read_to_string(input_file_name).expect(&message);
    input
}

fn get_overlap_pairs(input: String) -> usize {
    let elf_pairs = input.split("\n").collect::<Vec<&str>>();
    let overlap_pairs = elf_pairs.iter().fold(0, |acc, &line| acc + pair_is_overlap(line));
    overlap_pairs
}

fn pair_is_overlap(line: &str) -> usize {
    let split_index = line.find(",").unwrap_or(0);
    let elf_sections = line.split_at(split_index);
    // println!("elf_sections={:?}", &elf_sections);
    let elf_one_secton = get_section_min_max(elf_sections.0);
    let elf_two_section = get_section_min_max(elf_sections.1);
    // println!("elf_one={:?}", &elf_one_secton);
    // println!("elf_two={:?}", &elf_two_section);
    // if do_sections_overlap_part_one(elf_one_secton, elf_two_section) {
    if do_sections_overlap_part_two(elf_one_secton, elf_two_section) {
        1
    } else {
        0
    }
}
fn do_sections_overlap_part_two(section_one: (usize, usize), section_two: (usize, usize)) -> bool {
    let one_min = section_one.0;
    let one_max = section_one.1;
    let two_min = section_two.0;
    let two_max = section_two.1;
    let is_there_any_overlap = one_max >= two_min && one_min <= two_max;
    is_there_any_overlap
}

// fn do_sections_overlap_part_one(section_one: (usize, usize), section_two: (usize, usize)) -> bool {
//     let one_min = section_one.0;
//     let one_max = section_one.1;
//     let two_min = section_two.0;
//     let two_max = section_two.1;
//     let does_one_contain_two = one_min <= two_min && one_max >= two_max;
//     let does_two_contain_one = two_min <= one_min && two_max >= one_max;
//     does_one_contain_two || does_two_contain_one
// }

fn get_section_min_max(section: &str) -> (usize, usize) {
    let clean_section = section.replace(",", "");
    let split_index = clean_section.find("-").unwrap_or(0);
    let elf_section = clean_section.split_at(split_index);
    // println!("elf_section={:?}", elf_section);
    let min = elf_section.0.parse::<usize>().unwrap();
    let max = elf_section.1.replace("-", "").parse::<usize>().unwrap();
    (min, max)
}
