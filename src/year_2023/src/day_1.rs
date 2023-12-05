use std::str::FromStr;

use anyhow::Result;

mod trie;
use trie::{Trie, TrieOption};

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

    let interim = (num1 << 3) + num1 + num1 + num2;

    // print!("=> {:?}", &interim);
    interim
}

pub fn process_calibration(file_contents: String) -> Result<u32> {
    let mut running_numbers: Vec<u32> = vec![];
    let mut total = 0;

    for ch in file_contents.chars() {
        if ch == '\n' {
            let line_total = process_line(&running_numbers);
            total += line_total;
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
        total += line_total;
    }

    Ok(total)
}

pub fn process_calibration_part2(file_contents: String, trie: &'static Trie<u32>) -> Result<u32> {
    let mut running_numbers: Vec<u32> = vec![];
    let mut running_number_word = String::new();
    let mut total = 0;

    // let mut line = 0;
    // let mut lines = file_contents.lines();

    for ch in file_contents.chars() {
        // print!("Letter->{}", ch);
        if ch == '\n' {
            // line += 1;
            // print!("Line:{}, nums: {:?} ", line, &running_numbers,);
            let line_total = process_line(&running_numbers);
            // println!(" => {}", lines.next().unwrap());
            total += line_total;
            running_numbers.clear();
            running_number_word.clear();
        } else {
            if ch.is_numeric() {
                running_numbers.push(ch.to_digit(10).expect("Should be a number"));
                running_number_word.clear();
                // println!("Found digit: {:?}", &running_numbers);
                continue;
            }

            let candidate_str = &format!("{running_number_word}{ch}");
            // println!("Checking number word:{}", candidate_str);

            match trie.get_value(candidate_str) {
                TrieOption::Value(Some(number)) => {
                    // println!("Found number: {}", &number);
                    running_numbers.push(number);
                    running_number_word.clear();
                    running_number_word.push(ch); // keep last matching char in case it matches
                }
                TrieOption::Empty | TrieOption::Value(None) => {
                    running_number_word.push(ch);
                    // println!(
                    //     "Found prefix, Running number word now is: {}",
                    //     &running_number_word
                    // );
                }
                TrieOption::None => {
                    if candidate_str.len() > 1 {
                        running_number_word = String::from_str(&running_number_word[1..]).unwrap();
                        // running_number_word.clear();
                        running_number_word.push(ch);
                        // println!("Clearing Running number word, but keeping: {}", ch);
                    } else {
                        // println!("Clearing Running number word");
                        running_number_word.clear();
                    }
                }
            }
        }
    }

    // println!("Final running_number_word:{}", &running_number_word);
    // if running_number_word.len() > 0 {
    //     if let Some(number) = trie.get_value(&running_number_word) {
    //         running_numbers.push(number);
    //     }
    // }

    // println!("Final running_numbers:{:?}", &running_numbers);
    if running_numbers.len() > 0 {
        let line_total = process_line(&running_numbers);
        total += line_total;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::sync::OnceLock;
    use trie::Trie;

    fn init_trie() -> &'static Trie<u32> {
        static TRIE: OnceLock<Trie<u32>> = OnceLock::new();
        TRIE.get_or_init(|| {
            let mut t: Trie<u32> = Trie::new();
            t.add_string(&"one", 1);
            t.add_string(&"two", 2);
            t.add_string(&"three", 3);
            t.add_string(&"four", 4);
            t.add_string(&"five", 5);
            t.add_string(&"six", 6);
            t.add_string(&"seven", 7);
            t.add_string(&"eight", 8);
            t.add_string(&"nine", 9);

            t
        })
    }

    #[test]
    fn part2_final() {
        let start = std::time::Instant::now();

        let mut file = File::open("src/fixtures/day_1.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        let number = process_calibration_part2(file_contents, init_trie());

        assert_eq!(number.unwrap(), 54985u32);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn part2_issue3() {
        let test = r#"xrdnlbmtdeightone3threeeighttwo
bnnqzcfoneeight2hhdfkrrqzt
342tlmjgtfcnine"#
            .to_owned();

        let number = process_calibration_part2(test, init_trie());

        assert_eq!(133, number.unwrap());
    }

    #[test]
    fn part2_issue1() {
        let test = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#
            .to_owned();

        let number = process_calibration_part2(test, init_trie());

        assert_eq!(281, number.unwrap());
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
    fn part1() {
        let mut file = File::open("src/fixtures/day_1.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        let number = process_calibration(file_contents);

        assert_eq!(number.unwrap(), 55130_u32);
    }
}
