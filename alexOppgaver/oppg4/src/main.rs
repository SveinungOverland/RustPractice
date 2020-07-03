#![feature(generators)]

use gen_iter::gen_iter;
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
    gen_iter!({
        let (mut a, mut b) = (999, 999);
        loop {
            yield a * b;
            b = b - 1;
            if b == 0 {
                a = a - 1;
                b = a;
            }
            if a == 0 {
                return
            }
        }
    })
    .filter(|&x| check_if_palindrome(x))
    .max()
    .unwrap()
}

fn check_if_palindrome(number: i32) -> bool {
    let temp = number.to_string();
    return temp == temp.chars().rev().collect::<String>()
}