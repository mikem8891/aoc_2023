 pub fn trebuchet_calibration(cal_doc: &str) -> u64 {
     let mut sum = 0;
     for line in cal_doc.lines() {
         let mut nums = line.chars().filter(|c| c.is_ascii_digit());
         let first_digit = nums.next().unwrap();
         let last_digit = nums.last().unwrap_or(first_digit);
         let cal_val: String = [first_digit, last_digit].iter().collect();
         let cal_val: u64 = cal_val.parse().unwrap();
         sum += cal_val;
     }
     sum
 }
 
 pub fn trebuchet_calibration_p2(cal_doc: &str) -> Option<u32> {
    const NUM_WORDS: [(u8, &str); 9] = [
        (1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five"),
        (6, "six"), (7, "seven"), (8, "eight"), (9, "nine")
    ];
    let mut sum = 0;
    for line in cal_doc.lines() {
        let num_at = |i| Some(line[i..].as_bytes()[0] & 0x0F);
        let [mut first_digit, mut last_digit]: [Option<u8>; 2] = [None, None];
        let [mut begin, mut end] = [0, line.len()];
        if let Some(index) = &line.find(char::is_ascii_digit) {
            end = index + 1;
            first_digit = num_at(index);
        }
        if let Some(index) = &line.rfind(char::is_ascii_digit) {
            begin = index;
            last_digit = num_at(index);
        }
        for (num, word) in NUM_WORDS {
            if let Some(index) = line[..end].find(word) {
                end = index + word.len();
                first_digit = Some(num);
            }
            if let Some(index) = line[begin..].rfind(word) {
                begin += index;
                last_digit = Some(num);
            }
        }
        let two_digit_num = 10 * first_digit? + last_digit?;
        sum += two_digit_num as u32;
    }
    Some(sum)
 }
 
 #[cfg(test)]
 mod test{
    use super::*;

    #[test]
    fn example(){
        let input = 
r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let sum = trebuchet_calibration(input);
        assert_eq!(sum, 142, "sum is {sum}");
    }
    
    #[test]
    fn example_p2(){
        let input = 
r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let sum = trebuchet_calibration_p2(input).unwrap();
        assert_eq!(sum, 281, "sum is {sum}");
    }
}