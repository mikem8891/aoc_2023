use std::ops::Range;
use std::collections::HashMap;
use crate::util::*;

pub fn part_nums(eng_schem: &str) -> u64 {
    let mut sum = 0;
    let eng_schem: Vec<&str> = eng_schem.lines().collect();
    let is_in_range = |r: isize, c: isize| { 
        (r >= 0 && (r as usize) < eng_schem.len()) &&
        (c >= 0 && (c as usize) < eng_schem[r as usize].len())
    };
    for num_row in 0..(eng_schem.len()) {
        let mut end = 0;
        'num: while let Some(mut start) = eng_schem[num_row][end..].find(is_digit) {
            start += end;
            end = start + eng_schem[num_row][start..].find(is_not_digit).unwrap_or(eng_schem[num_row].len() - start);
            let row = num_row as isize;
            let ranges: [(isize, Range<isize>); 4] = {
                let (start, end) = (start as isize, end as isize);
                [
                    (row - 1, (start - 1)..(end + 1)),
                    (row    , (start - 1)..(start  )),
                    (row    , (end      )..(end + 1)),
                    (row + 1, (start - 1)..(end + 1))
                ]
            };
            for (r, c_range) in ranges {
                for c in c_range {
                    if is_in_range(r, c) {
                        let (r, c) = (r as usize, c as usize);
                        if eng_schem[r].as_bytes()[c] != b'.' {
                            sum += eng_schem[num_row][start..end]
                                .parse::<u64>().unwrap();
                            continue 'num;
                        }
                    }
                }
            }
        }
    }
    sum
}

pub fn part_2(eng_schem: &str) -> u64 {
    let mut sum = 0;
    let eng_schem: Vec<&str> = eng_schem.lines().collect();
    let mut gears: HashMap<[usize; 2], Vec<u64>> = HashMap::new();
    let is_in_range = |r: isize, c: isize| { 
        (r >= 0 && (r as usize) < eng_schem.len()) &&
        (c >= 0 && (c as usize) < eng_schem[r as usize].len())
    };
    for num_row in 0..(eng_schem.len()) {
        let mut end = 0;
        'num: while let Some(mut start) = eng_schem[num_row][end..].find(is_digit) {
            start += end;
            end = start + eng_schem[num_row][start..].find(is_not_digit).unwrap_or(eng_schem[num_row].len() - start);
            let row = num_row as isize;
            let ranges: [(isize, Range<isize>); 4] = {
                let (start, end) = (start as isize, end as isize);
                [
                    (row - 1, (start - 1)..(end + 1)),
                    (row    , (start - 1)..(start  )),
                    (row    , (end      )..(end + 1)),
                    (row + 1, (start - 1)..(end + 1))
                ]
            };
            for (r, c_range) in ranges {
                for c in c_range {
                    if is_in_range(r, c) {
                        let (r, c) = (r as usize, c as usize);
                        if eng_schem[r].as_bytes()[c] == b'*' {
                            let key = [r, c];
                            let elem = eng_schem[num_row][start..end]
                                .parse::<u64>().unwrap();
                            gears.entry(key).and_modify(|v| v.push(elem)).or_insert(vec![elem]);
                            continue 'num;
                        } else if eng_schem[r].as_bytes()[c] != b'.' {
                            continue 'num;
                        }
                    }
                }
            }
        }
    }
    for (_gear, values) in gears {
        if values.len() == 2 {
            sum += values.iter().product::<u64>();
        }
    }
    sum
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input =
r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(4361, part_nums(input));
    }

    #[test]
    fn example_2() {
        let input =
r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(467835, part_2(input));
    }
}