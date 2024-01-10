use std::fs;

mod day1;

fn main() {
    let file_path = "./inputs/day1.txt";
    let contents = fs::read_to_string(file_path).expect("Please provide valid file name");
    println!("{contents}");
}
