use anyhow::Result;

mod trie;
use trie::Trie;

pub fn process_line(line_numbers: &Vec<u32>) -> u32 {
    let num1: u32;
    let num2: u32;

    if line_numbers.len() < 2 {
        num1 = line_numbers[0];
        num2 = num1;
    } else {
        num1 = line_numbers[0];
        num2 = line_numbers[line_numbers.len() - 1];
    }

    let interim = (10 * num1) + num2;

    println!("Line process: {:?}, => {:?}", &(num1, num2), &interim);
    interim
}

pub fn process_calibration(file_contents: String) -> Result<u32> {
    let mut running_numbers: Vec<u32> = vec![];
    let mut total = 0;

    for ch in file_contents.chars() {
        if ch == '\n' {
            let line_total = process_line(&running_numbers);
            total = total + line_total;
            running_numbers.clear();
        } else {
            if ch.is_numeric() {
                running_numbers.push(ch.to_digit(10).expect("Should be a number"));
                // println!("{:?}", &running_numbers);
            }
        }
    }

    if running_numbers.len() > 0 {
        let line_total = process_line(&running_numbers);
        total = total + line_total;
    }

    Ok(total)
}

pub fn process_calibration_part2(file_contents: String, trie: Trie<u32>) -> Result<u32> {
    let mut running_numbers: Vec<u32> = vec![];
    let mut running_number_word = String::new();
    let mut total = 0;

    for ch in file_contents.chars() {
        if ch == '\n' {
            // println!("Processing line nums: {:?}", &running_numbers);
            let line_total = process_line(&running_numbers);
            total = total + line_total;
            running_numbers.clear();
            running_number_word.clear();
        } else {
            if ch.is_numeric() {
                running_number_word.clear();
                running_numbers.push(ch.to_digit(10).expect("Should be a number"));
                continue;
                // println!("Found digit: {:?}", &running_numbers);
            } else {
                // println!(
                //     "Checking number word:{}",
                //     &format!("{running_number_word}{ch}")
                // );

                let candidate_str = &format!("{running_number_word}{ch}");

                if let Some(number) = trie.get_value(candidate_str) {
                    // found a number word, save to number list and clear
                    // println!("Found number: {}", &number);
                    running_numbers.push(number);
                    running_number_word.clear();
                } else if trie.has_string(candidate_str) {
                    running_number_word.push(ch);
                    // println!("Running number word now is: {}", &running_number_word);
                } else {
                    // println!("Clearing Running number word: {}", &ch);
                    running_number_word.clear();
                    running_number_word.push(ch);
                }
            }
        }
    }

    // println!("Final running_number_word:{}", &running_number_word);
    if running_number_word.len() > 0 {
        if let Some(number) = trie.get_value(&running_number_word) {
            running_numbers.push(number);
        }
    }

    // println!("Final running_numbers:{:?}", &running_numbers);
    if running_numbers.len() > 0 {
        let line_total = process_line(&running_numbers);
        total = total + line_total;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use trie::Trie;

    #[test]
    fn part2_issue1() {
        let test = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#
        .to_owned();
        //         let test = r#"eightwothree
        // 7pqrstsixteen"#
        //             .to_owned();

        let mut t: Trie<u32> = Trie::new();

        let one = "one";
        let two = "two";
        let three = "three";
        let four = "four";
        let five = "five";
        let six = "six";
        let seven = "seven";
        let eight = "eight";
        let nine = "nine";

        t.add_string(&one, 1);
        t.add_string(&two, 2);
        t.add_string(&three, 3);
        t.add_string(&four, 4);
        t.add_string(&five, 5);
        t.add_string(&six, 6);
        t.add_string(&seven, 7);
        t.add_string(&eight, 8);
        t.add_string(&nine, 9);

        let number = process_calibration_part2(test, t);

        assert_eq!(281, number.unwrap());
    }

    #[test]
    fn part2_smoke() {
        let mut t: Trie<u32> = Trie::new();

        let one = "one".to_lowercase();
        let two = "two".to_lowercase();
        let three = "three".to_lowercase();
        let four = "four".to_lowercase();
        let five = "five".to_lowercase();
        let six = "six".to_lowercase();
        let seven = "seven".to_lowercase();
        let eight = "eight".to_lowercase();
        let nine = "nine".to_lowercase();

        t.add_string(&one, 1);
        t.add_string(&two, 2);
        t.add_string(&three, 3);
        t.add_string(&four, 4);
        t.add_string(&five, 5);
        t.add_string(&six, 6);
        t.add_string(&seven, 7);
        t.add_string(&eight, 8);
        t.add_string(&nine, 9);

        let mut file = File::open("src/fixtures/day_1.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        let number = process_calibration_part2(file_contents, t);

        assert_eq!(number.unwrap(), 55130_u32);
    }

    #[test]
    fn part_1_smoke() {
        let test_input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#
            .to_owned();

        let number = process_calibration(test_input).unwrap();
        assert_eq!(number, 142);
    }

    #[test]
    fn part_1_test_process_file() {
        let mut file = File::open("src/fixtures/day_1.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        let number = process_calibration(file_contents);

        assert_eq!(number.unwrap(), 55130_u32);
    }
}
