use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use const_format::pmr::Ok;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut lid1: Vec<i32> = vec![];
        let mut lid2: Vec<i32> = vec![];
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                lid1.push(nums[0]);
                lid2.push(nums[1]);
            }
        });
        lid1.sort();
        lid2.sort();
        let mut answer = 0;
        for i in 0..lid1.len() {
            answer += (lid1[i] - lid2[i]).abs();
        }
        Ok(answer as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lid1: Vec<i32> = vec![];
        let mut map2: HashMap<i32, i32> = HashMap::new();
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                lid1.push(nums[0]);
                if !map2.contains_key(&nums[1]) {
                    map2.insert(nums[1], 0);
                }
                map2.insert(nums[1], map2.get(&nums[1]).unwrap() + 1);
            }
        });
        let mut answer = 0;
        for i in 0..lid1.len() {
            if let Some(v) = map2.get(&lid1[i]) {
                answer += (lid1[i] * v);
            }
        }
        Ok(answer as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
