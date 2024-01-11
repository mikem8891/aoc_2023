use std::fs;

mod day1;
mod day2;

fn main() {
    println!("Day 1");
    let file_path = "./inputs/day1.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Day 1 input not found");
    println!("sum is {}", day1::trebuchet_calibration(&contents));
    println!("Day 2");
    let file_path = "./inputs/day2.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Day 2 input not found");
    println!("sum is {}", day2::snow_island_game(&contents));
}
