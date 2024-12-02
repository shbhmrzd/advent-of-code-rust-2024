use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use const_format::pmr::Ok;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut count = 0;

        // TODO: Solve Part 1 of the puzzle
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                if part1_helper(&nums) {
                    count += 1;
                }
            }
        });

        Ok(count as usize)
    }

    fn part1_helper(vec: &Vec<i32>) -> bool {
        let mut is_increasing = false;
        let mut is_decreasing = false;
        for i in 0..vec.len() - 1 {
            if vec[i] > vec[i + 1] {
                is_decreasing = true;
            } else if vec[i] < vec[i + 1] {
                is_increasing = true;
            } else {
                return false;
            }
            if (vec[i] - vec[i + 1]).abs() > 3 {
                return false;
            }
        }
        return is_increasing ^ is_decreasing;
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();

                if part2_helper(&nums) {
                    answer += 1;
                }
            }
        });

        Ok(answer as usize)
    }

    fn part2_helper(vec: &Vec<i32>) -> bool {
        if part1_helper(&vec) || vec.len() < 3 {
            return true;
        }

        let mut diff = vec![];
        let mut count = 0;
        let mut nc = 0;
        let mut pc = 0;
        let mut zc = 0;
        for i in 0..vec.len() - 1 {
            let num = vec[i] - vec[i + 1];
            if num.abs() > 3 {
                count += 1;
            }

            if num == 0 {
                zc += 1;
                count += 1;
            } else if num < 0 {
                nc += 1;
            } else {
                pc += 1;
            }
            diff.push(num);
        }

        if (pc == diff.len() || nc == diff.len()) && count == 0 {
            return part1_helper(&vec);
        }

        if zc > 1 || count > 2 || (pc > 2 && nc > 2) {
            return false;
        }

        let dir = if pc > nc { 1 } else { -1 };


        // If the diff at some index is troublemsome, we need to remove that index
        // from the parent vector and check if the resulting vector is valid
        // or the next index could be troublesome too
        let mut idx: i32 = -1;
        for i in 0..diff.len() {
            if (diff[i] * dir < 0 || (diff[i].abs() > 3 || diff[i].abs() == 0)) {
                idx = i as i32;

                if idx == 0 {
                    return part1_helper(&vec[1..].to_vec()) || part1_helper(&vec[..1].iter().chain(&vec[2..]).copied().collect::<Vec<_>>());
                } else if idx == diff.len() as i32 - 1 {
                    return part1_helper(&vec[..vec.len() - 1].to_vec()) || part1_helper(&vec[..vec.len()-2].iter().chain(&vec[vec.len()-1..]).copied().collect::<Vec<_>>());
                } else {
                    return part1_helper(&vec[..idx as usize].iter().chain(&vec[idx as usize + 1..]).copied().collect::<Vec<_>>()) || part1_helper(&vec[..idx as usize + 1].iter().chain(&vec[idx as usize + 2..]).copied().collect::<Vec<_>>());
                }
            }
        }
        false
    }

    fn part2_helper_bruteforce(vec: &Vec<i32>) -> bool {
        if vec.len() < 3 {
            return true; // A vector with fewer than 3 elements is always valid
        }

        if part1_helper(vec) {
            return true; // Already valid
        }

        // Calculate differences and counts
        let mut zc = 0; // Count of zero differences
        let mut diff = vec![];

        for i in 0..vec.len() - 1 {
            let num = vec[i] - vec[i + 1];
            if num == 0 {
                zc += 1;
            }
            diff.push(num);
        }

        // If more than one zero difference, we cannot fix it
        if zc > 1 {
            return false;
        }

        // Try removing one element to see if it fixes the sequence
        for i in 0..vec.len() {
            let is_valid = if i == 0 {
                part1_helper(&vec[1..].to_vec())
            } else if i == vec.len() - 1 {
                part1_helper(&vec[..vec.len() - 1].to_vec())
            } else {
                part1_helper(
                    &vec[..i]
                        .iter()
                        .chain(vec[i + 1..].iter())
                        .copied()
                        .collect::<Vec<_>>(),
                )
            };

            if is_valid {
                return true;
            }
        }

        false // No single removal made the sequence valid
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
