use std::file;

mod day1;

fn main() {
    let file_path = "input/day1.txt";
    let contents = fs::read_to_string(file_path);
    println!("{contents}");
}
