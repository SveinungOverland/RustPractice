use std::time::{ Instant };

/**
 * Largest prime factor
 * 
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 */

/**
 * Output:
 * The naive approach took:  0.005380901s, and provided the answer: 6857
 */

const NUMBER_FLOAT: f64 = 600851475143.0;
const NUMBER_INT: i64 = 600851475143;

fn main() {
    let now = Instant::now();
    let naive_answer = naive();
    println!("The naive approach took:  {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), naive_answer);
}

fn naive() -> i64 {
    let number_sqrt: i64 = NUMBER_FLOAT.sqrt().ceil() as i64;
    (2..number_sqrt).rev().into_iter().find(|&x| {
        if NUMBER_INT % x == 0 {
            let sqrt: i64 = (x as f64).sqrt().ceil() as i64;
            return (2..sqrt).into_iter().find(|&y| x % y == 0).is_none()
        }
        false
    }).unwrap()
}