extern crate utilities;

use utilities::import;


const PATH: &str = "inputs/input0.txt";

fn main() {
    let input = import::get_input(PATH);
    println!("input:\n{}", input);
}
