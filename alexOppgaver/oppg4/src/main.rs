use std::time::{ Instant };

/**
 * Largest palindrome product
 * 
 * A palindromic number reads the same both ways.
 * The largest palindrome made from the product 
 * of two 2-digit numbers is 9009 = 91 × 99.
 * 
 * Find the largest palindrome made from the product 
 * of two 3-digit numbers.
 */

fn main() {
    let now = Instant::now();
    let naive_answer = naive();
    println!("The naive approach took:  {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), naive_answer);
}

fn naive() -> i32 {
    // TODO: DO
    1
}