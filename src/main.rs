use std::fs;

mod day1;
mod day2;

fn main() {
    println!("Day 1");
    let file_path = "./inputs/day1.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Day 1 input not found");
    println!("part 1: sum is {}", day1::trebuchet_calibration(&contents));
    println!("part 2: sum is {}", day1::trebuchet_calibration_p2(&contents).unwrap());
    println!("Day 2");
    let file_path = "./inputs/day2.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Day 2 input not found");
    println!("part 1: sum is {}", day2::snow_island_game(&contents));
    println!("part 2: sum is {}", day2::snow_island_game_p2(&contents));
    println!("Day 3");
    let file_path = "./inputs/day3.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Day 3 input not found");
}
