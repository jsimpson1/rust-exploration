use std::collections::HashSet;
use utilities::import;

const PATH: &str = "inputs/input1.txt";

fn main() {
    let input = import::get_input(PATH);
    println!("inputs='{}'", input);
    let datastream = &input;
    let index_of_marker: usize = find_index_from_datastream_buffer(datastream);
    println!("index of marker={}", index_of_marker)
}

fn find_index_from_datastream_buffer(datastream: &str) -> usize {
    let characters: Vec<char> = datastream.chars().collect();
    let marker_size = 4;
    let max_index = characters.len() - marker_size;

    for index in 0..max_index {
        let potential_marker = &characters[index..index+marker_size];
        println!("potential_marker={:?}", potential_marker);
        let unique_hash: HashSet<&char> = HashSet::from_iter(potential_marker.iter());
        if unique_hash.len() == marker_size {
            let value = index + marker_size;
            return value
        }
        println!("unique_hash={:?}", unique_hash);
    }
    return 0
}
