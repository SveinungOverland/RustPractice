#![feature(generators)]

// use gen_iter::gen_iter;
use rayon::prelude::*;
use std::time::{ Instant };
// use std::collections::VecDeque;
use std::collections::HashSet;


/**
 * Counting fractions
 * 
 * Consider the fraction, n/d, where n and d are positive integers. 
 * If n<d and HCF(n,d)=1, it is called a reduced proper fraction.
 * 
 * If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
 * 
 * 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
 * 
 * It can be seen that there are 21 elements in this set.
 * How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?
 */

/**
 * Output:
 * 
 */ 

fn main() {
    let now = Instant::now();
    let naive_answer = naive();
    println!("The naive approach took:  {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), naive_answer);
}

fn naive() -> usize {
    const NR_SETS: usize = 200;
    rayon::ThreadPoolBuilder::new().num_threads(3).build_global().unwrap();
    let mut set = HashSet::<i32>::new();
    let mut current_index = 2;
    let mut indexes_left = 1_000_000;
    
    for x in 1..=NR_SETS {
        println!("Creating: {}", x);
        let range_limit = if indexes_left / 2 > 100_000 { 100_000 } else { indexes_left / 2 };
        let range_top = if x == NR_SETS { 1_000_000 } else { current_index + range_limit };
        indexes_left = indexes_left - range_limit;
        let temp_set = (current_index..range_top)
            .into_par_iter()
            .flat_map(|x| { (1..x).into_par_iter().map(move |y| ((y as f64 / x as f64) * 1000f64) as i32) })
            .collect::<HashSet<i32>>();
        println!();
        set = set.union(&temp_set).map(|&x| x).collect::<HashSet<i32>>();
        current_index = current_index + range_limit;
    }

    set.len()
}