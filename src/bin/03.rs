use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use const_format::pmr::Ok;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut sum = 0;
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                // Regex to match valid `mul(a,b)` instructions
                let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();



                for cap in re.captures_iter(&line) {
                    // Extract the two numbers
                    let a: i32 = cap[1].parse().unwrap();
                    let b: i32 = cap[2].parse().unwrap();

                    // Compute and add the product
                    sum += a * b;
                }

            }
        });

        Ok(sum as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut sum = 0;
        let mut enabled = true;
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

               re.captures_iter(&line).for_each(|caps| {
                    if caps.get(1).is_some() {
                        let first: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                        let second: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                         if enabled {
                             println!("Regex: {} * {}", first, second);
                             sum += first * second
                         }
                    } else if caps.get(0).unwrap().as_str() == "do()" {
                        enabled = true
                    } else if caps.get(0).unwrap().as_str() == "don't()" {
                        enabled = false
                    } else {
                        unreachable!()
                    }
                });
            }
        });

        Ok(sum as usize)
    }

    fn part2_split<R: BufRead>(reader: R) -> Result<usize> {
        let mut sum = 0;
        let mut enabled = true;
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

                let do_splits =  line.split("do()").collect::<Vec<&str>>();
                do_splits.iter().for_each(|line| {
                    let dont_splits = line.split("don't()").collect::<Vec<&str>>();

                        re.captures_iter(dont_splits.get(0).unwrap()).for_each(|caps| {
                            if caps.get(1).is_some() {
                                let first: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                                let second: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                                println!("Split: {} * {}", first, second);
                                if enabled {sum += first * second}
                            }
                        });

                });
            }
        });

        Ok(sum as usize)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    let input_file_split = BufReader::new(File::open(INPUT_FILE)?);
    let result_split = time_snippet!(part2_split(input_file_split)?);
    println!("Result split = {}", result_split);
    //endregion

    Ok(())
}
