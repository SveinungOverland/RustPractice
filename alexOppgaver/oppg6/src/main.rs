use std::time::{ Instant };

/**
 * Sum square difference
 * 
 * The sum of the squares of the first ten natural numbers is,
 * 
 * 1^2+2^2+...+10^2=385
 * 
 * The square of the sum of the first ten natural numbers is,
 * 
 * (1+2+...+10)^2=55^2=3025
 * 
 * Hence the difference between the sum of the squares of the 
 * first ten natural numbers and the square of the sum is 3025−385=2640.
 * Find the difference between the sum of the squares of the 
 * first one hundred natural numbers and the square of the sum.
 */

/**
 * Output:
 * 
 * The naive approach took:  0.000000019s, and provided the answer: 25164150
 */ 

fn main() {
    let now = Instant::now();
    let naive_answer = naive();
    println!("The naive approach took:  {}s, and provided the answer: {}", now.elapsed().as_secs_f32(), naive_answer);
}

fn naive() -> i64 {
    (1..=100).sum::<i64>().pow(2) - (1..=100).into_iter().map(|x: i64| x.pow(2)).sum::<i64>()
}