/*
--- Day 9: Mirage Maintenance ---
You ride the camel through the sandstorm and stop where the ghost's maps told you to stop. The sandstorm subsequently subsides, somehow seeing you standing at an oasis!

The camel goes to get some water and you stretch your neck. As you look up, you discover what must be yet another giant floating island, this one made of metal! That must be where the parts to fix the sand machines come from.

There's even a hang glider partially buried in the sand here; once the sun rises and heats up the sand, you might be able to use the glider and the hot air to get all the way up to the metal island!

While you wait for the sun to rise, you admire the oasis hidden here in the middle of Desert Island. It must have a delicate ecosystem; you might as well take some ecological readings while you wait. Maybe you can report any environmental instabilities you find to someone so the oasis can be around for the next sandstorm-worn traveler.

You pull out your handy Oasis And Sand Instability Sensor and analyze your surroundings. The OASIS produces a report of many values and how they are changing over time (your puzzle input). Each line in the report contains the history of a single value. For example:

0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
To best protect the oasis, your environmental report should include a prediction of the next value in each history. To do this, start by making a new sequence from the difference at each step of your history. If that sequence is not all zeroes, repeat this process, using the sequence you just generated as the input sequence. Once all of the values in your latest sequence are zeroes, you can extrapolate what the next value of the original history should be.

In the above dataset, the first history is 0 3 6 9 12 15. Because the values increase by 3 each step, the first sequence of differences that you generate will be 3 3 3 3 3. Note that this sequence has one fewer value than the input sequence because at each step it considers two numbers from the input. Since these values aren't all zero, repeat the process: the values differ by 0 at each step, so the next sequence is 0 0 0 0. This means you have enough information to extrapolate the history! Visually, these sequences can be arranged like this:

0   3   6   9  12  15
  3   3   3   3   3
    0   0   0   0
To extrapolate, start by adding a new zero to the end of your list of zeroes; because the zeroes represent differences between the two values above them, this also means there is now a placeholder in every sequence above it:

0   3   6   9  12  15   B
  3   3   3   3   3   A
    0   0   0   0   0
You can then start filling in placeholders from the bottom up. A needs to be the result of increasing 3 (the value to its left) by 0 (the value below it); this means A must be 3:

0   3   6   9  12  15   B
  3   3   3   3   3   3
    0   0   0   0   0
Finally, you can fill in B, which needs to be the result of increasing 15 (the value to its left) by 3 (the value below it), or 18:

0   3   6   9  12  15  18
  3   3   3   3   3   3
    0   0   0   0   0
So, the next value of the first history is 18.

Finding all-zero differences for the second history requires an additional sequence:

1   3   6  10  15  21
  2   3   4   5   6
    1   1   1   1
      0   0   0
Then, following the same process as before, work out the next value in each sequence from the bottom up:

1   3   6  10  15  21  28
  2   3   4   5   6   7
    1   1   1   1   1
      0   0   0   0
So, the next value of the second history is 28.

The third history requires even more sequences, but its next value can be found the same way:

10  13  16  21  30  45  68
   3   3   5   9  15  23
     0   2   4   6   8
       2   2   2   2
         0   0   0
So, the next value of the third history is 68.

If you find the next value for each history in this example and add them together, you get 114.

Analyze your OASIS report and extrapolate the next value for each history. What is the sum of these extrapolated values?
*/

use anyhow::{anyhow, Context, Result};
use std::str::FromStr;

struct Measurement(Vec<isize>);

impl Measurement {
    fn new(values: Vec<isize>) -> Self {
        Self(values)
    }

    fn last(&self) -> isize {
        *self.0.last().unwrap()
    }

    fn first(&self) -> isize {
        *self.0.first().unwrap()
    }

    fn reduce_all(&self) -> Vec<Measurement> {
        std::iter::successors(Some(self.reduce_measurement()), |next| {
            (!next.is_zero_measurement()).then(|| next.reduce_measurement())
        })
        .collect()
    }

    fn reduce_measurement(&self) -> Measurement {
        Measurement::new(
            self.0
                .iter()
                .zip(self.0.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect(),
        )
    }

    fn is_zero_measurement(&self) -> bool {
        self.0.iter().all(|&n| n == 0)
    }

    fn next(&self) -> isize {
        self.last()
            + self
                .reduce_all()
                .iter()
                .rfold(0isize, |acc, mem| acc + mem.last())
    }

    fn prev(&self) -> isize {
        self.first()
            - self
                .reduce_all()
                .iter()
                .rfold(0isize, |acc, mem| mem.first() - acc)
    }
}

impl FromStr for Measurement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::new(
            s.split_ascii_whitespace()
                .map(|s| {
                    s.parse::<isize>()
                        .with_context(|| anyhow!("Could not parse number"))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub struct Report(Vec<Measurement>);

impl Report {
    pub fn total_next(&self) -> isize {
        self.0.iter().map(Measurement::next).sum()
    }

    pub fn total_prev(&self) -> isize {
        self.0.iter().map(Measurement::prev).sum()
    }
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.parse::<Measurement>()
                        .with_context(|| anyhow!("Invalid measurement"))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn load_file() -> String {
        let mut file = File::open("src/fixtures/day9.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);
        file_contents
    }

    #[test]
    fn day9_part2_file() {
        let start = std::time::Instant::now();
        let sample = load_file();
        let report = sample.parse::<Report>().unwrap();
        let total_next_numbers = report.total_prev();

        assert_eq!(total_next_numbers, 1124);
        // 1923760368 to high

        println!("Elapsed time: {:?}", start.elapsed());
    }

    #[test]
    fn day9_part2_sample() {
        let start = std::time::Instant::now();

        let sample = r#"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"#;

        let report = sample.parse::<Report>().unwrap();
        let total_next_numbers = report.total_prev();

        assert_eq!(total_next_numbers, 2);

        println!("Elapsed time: {:?}", start.elapsed());
    }

    #[test]
    fn day9_part1_file() {
        let start = std::time::Instant::now();
        let sample = load_file();
        let report = sample.parse::<Report>().unwrap();
        let total_next_numbers = report.total_next();

        assert_eq!(total_next_numbers, 1921197370);
        // 1923760368 to high

        println!("Elapsed time: {:?}", start.elapsed());
    }

    #[test]
    fn day9_part1_sample() {
        let start = std::time::Instant::now();

        let sample = r#"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"#;

        let report = sample.parse::<Report>().unwrap();
        let total_next_numbers = report.total_next();

        assert_eq!(total_next_numbers, 114);

        // println!("sample_nums: {:?}", &next_numbers);
        println!("Elapsed time: {:?}", start.elapsed());
    }
}
