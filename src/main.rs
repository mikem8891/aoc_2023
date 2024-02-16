use std::fs;

mod util;
mod day1;
mod day2;
mod day3;
mod day4;
//mod day5;
mod day6;
mod day7;
//mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

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
    println!("part 1: sum is {}", day3::part_nums(&contents));
    println!("part 2: sum is {}", day3::part_2(&contents));
    println!("Day 4");
    let file_path = "./inputs/day4.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Day 4 input not found");
    let (part_1, part_2) = day4::solve(&contents);
    println!("part 1: sum is {}", part_1);
    println!("part 2: sum is {}", part_2);
//    day5::main();
    day6::main();
    day7::main();
//    day8::main();
    day9::main();
    day10::main();
    day11::main();
    day12::main();
}
