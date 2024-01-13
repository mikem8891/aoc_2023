use crate::util::*;

pub fn part_nums(eng_schem: &str) -> u64 {
    let mut sum = 0;
    let eng_schem: Vec<&str> = eng_schem.lines().collect();
    let is_in_range = |r: isize, c: isize| { 
        c >= 0 && (c as usize) < eng_schem[r].len() &&
        r >= 0 && (r as usize) < eng_schem.len()
    };
    let is_part_at = |r, c| {
        is_in_range(r, c) &&
        eng_schem[r].as_bytes()[c] != b'.';
    };
    for num_row in 0..eng_schem.len() {
        let mut end = 0;
        while let Some(start) = eng_schem[num_row][end..].find(is_digit) {
            end = start + eng_schem[num_row][start..].find(is_not_digit).unwrap();
            let is_part = 'find: {
                let ranges: [(isize, Range<isize>); 4] = [
                    (num_row - 1, (start - 1)..(end + 1)),
                    (num_row    , (start - 1)..(start  )),
                    (num_row    , (end      )..(end + 1)),
                    (num_row + 1, (start - 1)..(end + 1))
                ];
                for (r, c_range) in ranges {
                    for c in c_range {
                        if is_in_range(r, c) {
                            let (r, c) = (r as usize, c as usize);
                            if eng_schem[r].as_bytes()[c] != &b'.' {
                                break 'find true;
                            }
                        }
                    }
                }
                false
            };
            if is_part {
                sum += eng_schem[num_row][start..end].parse::<u64>().unwrap();
            }
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
}