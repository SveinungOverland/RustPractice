#![feature(generators)]

use gen_iter::gen_iter;
use std::time::{ Instant };

/**
 * 10001st prime
 * 
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 * What is the 10 001st prime number?
 */

/**
 * Output:
 * 
 * The naive approach took:  0.3260433s, and provided the answer: 104743
 */ 

fn main() {
    let now = Instant::now();
    let naive_answer = naive();
    println!("The naive approach took:  {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), naive_answer);
}

fn naive() -> i64 {
    // A prime number is a number only divisible by itself and 1
    gen_iter!({
        let mut i = 2;
        let mut prime_vec = vec![];
        loop { 
            if prime_vec.iter().find(|&x| i % x == 0).is_none() { 
                prime_vec.push(i);
                yield i;
            } 
            i = i + 1; 
        }
    }).take(10_001).last().unwrap()
}