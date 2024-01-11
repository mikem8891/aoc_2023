use std::fs;

mod day1;

fn main() {
    println!("Day 1");
    let mut file_path = "./inputs/day1.txt";
    let mut contents = fs::read_to_string(file_path)
        .expect("Day 1 input not found");

}
