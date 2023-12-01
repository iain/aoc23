use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn day_one() {
    let re = Regex::new(r"^([0-9])$").unwrap();

    let mut nums: Vec<Vec<i32>> = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./inputs/day_one.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(text) = line {
                println!("{}", text);
                let text = text.replace("one", "one1one");
                let text = text.replace("two", "two2two");
                let text = text.replace("three", "three3three");
                let text = text.replace("four", "four4four");
                let text = text.replace("five", "five5five");
                let text = text.replace("six", "six6six");
                let text = text.replace("seven", "seven7seven");
                let text = text.replace("eight", "eight8eight");
                let text = text.replace("nine", "nine9nine");

                let mut lin_nums: Vec<i32> = Vec::new();
                for c in text.chars() {
                    let w = c.to_string();
                    let result = re.captures(&w);
                    match result {
                        Some(caps) => lin_nums.push(caps[1].parse::<i32>().unwrap()),
                        None => (),
                    }
                }
                nums.push(lin_nums);
            }
        }
    }
    println!("{:?}", nums);
    let mut sum = 0;
    for row in nums {
        let output = format!("{}{}", row[0], row[row.len() - 1])
            .parse::<i32>()
            .unwrap();
        sum = sum + output;
        println!("{:?}", output);
    }
    println!("{:?}", sum)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
