use rand::Rng;
use std::collections::HashMap;

fn main() {
    // Create a sample of ints
    let mut rng = rand::rng();
    let mut numbers: Vec<i32> = Vec::new();
    for _i in 0..100 {
        numbers.push(rng.random_range(0..100));
    }

    // Sort the numbers
    numbers.sort();
    println!("Sample:\n{:?}", numbers);

    // Find the mode
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &number in &numbers {
        *counts.entry(number).or_insert(0) += 1;
    }

    match counts.values().max() {
        Some(1) => println!("No mode, all numbers occurs only once."),
        Some(num) => {
            for (&value, &count) in &counts {
                if count == *num {
                    println!("Mode: {:?}, Count: {}", value, count);
                }
            }
        },
        _ => println!("Could not compute mode."),
    }

    // Find the median
    match numbers.len() {
        len if len % 2 == 0 => {
            let left = numbers[len / 2];
            let right = numbers[len / 2 - 1];
            let median: f32 = (left + right) as f32 * 0.5;
            println!("Median: {}", median);
        },
        len if len % 2 == 1 => {
            println!("Median = {}", numbers[len / 2]);
        },
        _ => println!("Could not compute median."),
    }
}
