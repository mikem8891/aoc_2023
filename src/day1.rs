 fn trebuchet_calibration(cal_doc: &str) -> u64 {
     let mut sum = 0;
     for line in cal_doc.lines() {
         let mut nums = line.chars().filter(|c| c.is_digit(10));
         let first_digit = nums.next().unwrap();
         let last_digit = nums.last().unwrap_or(first_digit);
         let cal_val: String = [first_digit, last_digit].iter().collect();
         let cal_val: u64 = cal_val.parse().unwrap();
         sum += cal_val;
     }
     sum
 }
 
 #[cfg(test)]
 mod test{
    #[test]
    fn example(){
        input = 
r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let sum = trebuchet_calibration(input);
        println!("sum is {sum}");
        assert_eq!(sum, 142);
    }
 }