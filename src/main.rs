extern crate regex;

mod parser;
mod day01;
mod puzzles;

use std::fs::read_to_string;
use std::time::Instant;
use puzzles::*;

fn timed_run<F>(mut f: F) where F: FnMut(){
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    println!("Timing : {:?}", duration);
}

fn main() -> Result<(), std::io::Error> {
    {//Day 01
        let input = day01::Input::new(&read_to_string("day01.input")?);
        let mut part1 = day01::Part1::new(&input);
        let mut part2 = day01::Part2::new(&input);

        println!("Day 01 part 1");
        timed_run(||{part1.resolve();});
        println!("{}",part1);

        println!("Day 01 part 2");
        timed_run(||{part2.resolve();});
        println!("{}",part2);

    }
    {//Day 02
    
    }
    Ok(())
}
