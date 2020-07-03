#![feature(generators)]

use gen_iter::gen_iter;
use std::time::{ Instant };

/**
 * Smallest multiple
 * 
 * 2520 is the smallest number that can be divided by 
 * each of the numbers from 1 to 10 without any remainder.
 * 
 * What is the smallest positive number that is evenly 
 * divisible by all of the numbers from 1 to 20?
 */

/**
 * Output:
 * 
 * The naive approach took:  0.6109428s, and provided the answer: 232792560
 */ 

fn main() {
    let now = Instant::now();
    let naive_answer = naive();
    println!("The naive approach took:  {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), naive_answer);
}

fn naive() -> i32 {
    
    gen_iter!({ let mut x = 20; loop { yield x; x = x + 1; } })
    .find(|x| {
        (2..=20).rev().find(|y| x % y != 0).is_none()
    })
    .unwrap()
}