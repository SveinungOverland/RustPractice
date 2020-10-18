#![feature(generators)]

use gen_iter::gen_iter;
use rayon::prelude::*;
use std::time::{ Instant };
use std::collections::VecDeque;


/**
 * Consecutive prime sum
 * 
 * The prime 41, can be written as the sum of six consecutive primes:
 * 
 * 41 = 2 + 3 + 5 + 7 + 11 + 13
 * 
 * This is the longest sum of consecutive primes that adds to a prime below one-hundred.
 * 
 * The longest sum of consecutive primes below one-thousand that adds to a prime, 
 * contains 21 terms, and is equal to 953.
 * 
 * Which prime, below one-million, can be written as the sum of the most consecutive primes?
 */

/**
 * Output:
 * 
 * The naive approach took:  24.358475s, and provided the answer: 997651
 */ 

fn main() {
    let now = Instant::now();
    let naive_answer = naive();
    println!("The naive approach took:  {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), naive_answer);
}

fn naive() -> i64 {
    // A prime number is a number only divisible by itself and 1
    let primes = gen_iter!({
        let mut i = 2;
        let mut prime_vec: Vec<i64> = vec![];
        loop { 
            if prime_vec.iter().find(|&x| i % x == 0).is_none() { 
                prime_vec.push(i);
                print!("\r:{}", i);
                yield i;
            } 
            i = i + 1; 
        }
    }).take_while(|&x| x < 1_000_000).collect::<Vec<i64>>();
    let result = primes.par_iter().filter_map(|&x| {
        let mut consecutive_primes: VecDeque<i64> = VecDeque::new();
        let mut current_index = 0;
        loop {
            let current_sum = consecutive_primes.iter().sum::<i64>();
            if primes[current_index] == x { return None; }
            else if current_sum == x {
                return Some((x, consecutive_primes.len()));
            }
            if current_sum > x {
                consecutive_primes.pop_front().unwrap();
            } else if current_sum < x {
                consecutive_primes.push_back(primes[current_index]);
                current_index = current_index + 1;
            }
        }
    }).max_by_key(|x| x.1).unwrap();

    result.0
}