use std::collections::HashMap;

fn median_and_mode(numbers: Vec<i32>) -> (f64, i32) {
    let mut numbers = numbers;
    numbers.sort();
    let len = numbers.len();

    // Compute the median
    let median = if len % 2 == 0 {
        (numbers[len / 2] + numbers[len / 2 - 1]) as f64 / 2.0
    } else {
        numbers[len / 2] as f64
    };

    // Compute the mode
    let mut mode = 0;
    let mut mode_count = 0;
    let mut counts = HashMap::new();
    for &number in &numbers {
        *counts.entry(number).or_insert(0) += 1;
    }
    for (&number, &count) in &counts {
        if count > mode_count {
            mode = number;
            mode_count = count;
        }
    }

    (median, mode)
}
