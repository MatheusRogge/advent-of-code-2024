use std::io::{self, Read};

use regex::Regex;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;

    let mut result = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    for captures in regex.captures_iter(input.as_str()) {
        let left: i64 = captures.get(1).map(|e| e.as_str()).unwrap().parse()?;
        let right: i64 = captures.get(2).map(|e| e.as_str()).unwrap().parse()?;

        result += left * right;
    }

    println!("Result: {:?}", result);
    Ok(())
}
