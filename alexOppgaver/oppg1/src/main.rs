use std::time::{ Instant };

/**
 * Multiples of 3 and 5
 * 
 * If we list all the natural numbers below 10 
 * that are multiples of 3 or 5, we get 3, 5, 6 
 * and 9. The sum of these multiples is 23.
 * 
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

/**
 * Output:
 * The naive approach took:  0.000003794s, and provided the answer: 233168
 * The clever approach took: 0.000000841s, and provided the answer: 233168
 */

fn main() {
    let mut now = Instant::now();
    let naive_answer = naive();
    println!("The naive approach took:  {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), naive_answer);
    
    now = Instant::now();
    let clever_answer = clever();
    println!("The clever approach took: {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), clever_answer);
}

fn naive() -> i32 {
    // Naive approach: Iterate through all numbers and check if it
    (1..1_000)
        .into_iter()
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

fn clever() -> i32 {
    (0..1_000).step_by(5)
        .into_iter()
        .filter(|&x| x % 3 != 0)
    .chain((0..1_000).step_by(3))
    .sum()
}