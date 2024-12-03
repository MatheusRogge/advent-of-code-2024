use std::{collections::BinaryHeap, io};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in io::stdin().lines() {
        match line?.split_once("   ") {
            None => {},
            Some((left, right)) => {
                let left_value = left.parse::<i32>()?;
                let right_value = right.parse::<i32>()?;

                left_heap.push(left_value);
                right_heap.push(right_value);
            }
        }
    }

    let sorted_left = left_heap.into_sorted_vec();
    let sorted_right = right_heap.into_sorted_vec();

    let result: u32 = sorted_left.into_iter()
        .zip(sorted_right)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("Result: {}", result);
    Ok(())
}
