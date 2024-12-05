use std::io::{self, Read};

use regex::Regex;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result = 0;
    let mut enabled = true;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let regex = Regex::new(r"((do|don't)\(\))|(mul\(([0-9]{1,3}),([0-9]{1,3})\))")?;
    for captures in regex.captures_iter(input.as_str()) {
        let enable_instruction = captures.get(2).map(|e| e.as_str());
        match enable_instruction {
            Some("do") => enabled = true,
            Some("don't") => enabled = false,
            _ => {},
        }

        if !enabled {
            continue;
        }

        let left_param = captures.get(4).map(|e| e.as_str());
        let right_param = captures.get(5).map(|e| e.as_str());

        if let (Some(left), Some(right)) = (left_param, right_param) {
            let left: i64 = left.parse()?;
            let right: i64 = right.parse()?;

            result += left * right;
        }
    }

    println!("Result: {:?}", result);
    Ok(())
}
