use std::fs::File;
use std::io::{ BufRead, BufReader, BufWriter };
use std::time::{ Instant };
use rayon::prelude::*;

/**
 * Goal: Read in a file full of strings, 
 * sort them alphabetically, write the 
 * sorted contents to a new file.
 */

fn main() {
    let now = Instant::now();
    do_stuff();
    println!("The stuff took {} microseconds", now.elapsed().as_micros());
}


fn do_stuff() {
    let reader = BufReader::with_capacity(400_000, File::open("strings.txt").unwrap());
    let writer = BufWriter::with_capacity(400_000, File::create("result.txt").unwrap());
    let mut lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    lines.par_sort();
    lines.iter()
        .for_each(|line| writer.write_all(mut buf: &[u8])(format_args!("{}\n", line)));
}