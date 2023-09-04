use std::collections::HashSet;
use std::fs;
use std::iter::Iterator;

// const PUZZLE_PART: u32 = 0;
const PUZZLE_PART: u32 = 1;

fn main() {
    let input = get_input(PUZZLE_PART);
    println!("input:\n{}", input);
    let calculated_sum = calculate_items_sum(input);
    println!("calculated_sum={}", calculated_sum)
}

fn get_input(part: u32) -> String {
    let input_file_name = format!("src/input_part{}.txt", part);
    let message = format!("Should be getting input from a file: {}", input_file_name);
    let input = fs::read_to_string(input_file_name).expect(&message);
    input
}

fn calculate_items_sum(input: String) -> usize {
    let rucksacks = input.split("\n").collect::<Vec<&str>>();
    calculate_sum_for_rucksacks(rucksacks)
}

fn calculate_sum_for_rucksacks(rucksacks: Vec<&str>) -> usize {
    rucksacks
        .iter()
        .fold(
            0,
            |acc, rucksack_items| acc + calculate_rucksack_value(rucksack_items)
        )
}

fn calculate_rucksack_value(rucksack_items: &str) -> usize {
    let middle_index = rucksack_items.len() / 2;
    let compartments = rucksack_items.split_at(middle_index);
    let first_compartment = compartments.0.chars().collect();
    let second_compartment = compartments.1.chars().collect();
    // println!("first_compartment={:?}", &first_compartment);
    // println!("second_compartment={:?}", &second_compartment);
    let item = get_item_intersect(first_compartment, second_compartment);
    // println!("item={}", &item);
    let value = get_priority_value(item);
    value
}

fn get_item_intersect<'a>(first_compartment: Vec<char>, second_compartment: Vec<char>) -> char {
    let compartment_one = first_compartment.into_iter().collect::<HashSet<char>>();
    let compartment_two = second_compartment.into_iter().collect::<HashSet<char>>();
    let intersect = compartment_one.intersection(&compartment_two);
    let intersect_set = intersect.collect::<Vec<_>>();
    // println!("intersect_set={:?}", &intersect_set);
    let item = intersect_set[0].clone();
    // println!("item={}", item);
    item
}

fn get_priority_value(item: char) -> usize {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    let priority = letters.iter().position(|&c| c == item ).unwrap() + 1;
    println!("priority={:?}", priority);
    return priority

}
