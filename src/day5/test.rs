const EXAMPLE_SOLUTION: [&str; 2] = ["35", ""];

use super::*;

#[test]
fn example_1(){
    let path = format!("./src/day{DAY_NUM}/example_1.txt");
    let input = std::fs::read_to_string(path).unwrap();
    assert_eq!(solve(&input)[0], EXAMPLE_SOLUTION[0]); 
}

#[test]
fn example_2(){
    let path = format!("./src/day{DAY_NUM}/example_2.txt");
    let input = std::fs::read_to_string(path).unwrap();
    assert_eq!(solve(&input)[1], EXAMPLE_SOLUTION[1]); 
}