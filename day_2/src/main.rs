use std::{collections::VecDeque, io};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result = 0;

    for line in io::stdin().lines() {
        let numbers: VecDeque<i32> = line?
            .split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();

        let mut valid = true;
        let mut last_direction = 0;
        let mut iter = numbers.iter().peekable();

        while let Some(current) = iter.next() {
            match iter.peek() {
                None => break,
                Some(next) => {
                    let sub = *current - *next;
                    let direction = sub.signum();

                    if last_direction == 0 {
                        last_direction = direction;
                    }

                    if direction != last_direction || !(1..=3).contains(&sub.abs()) {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if valid {
            result += 1;
        }
    }

    println!("Result: {}", result);
    Ok(())
}
