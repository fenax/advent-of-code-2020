extern crate regex;
extern crate itertools;
mod parser;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod puzzles;

use std::fs::read_to_string;
use std::time::Instant;
use puzzles::*;
use std::vec::Vec;

macro_rules! one_day {
    ($day:ident) => 
   {
        println!("{}",stringify!($day));

        let i_string = read_to_string(concat!(stringify!($day),".input"))?;
        let input =timed_run(||{$day::Input::new(&i_string)});
       
        let mut part1 = $day::Part1::new(&input);
        let mut part2 = $day::Part2::new(&input);

        println!(" part 1");
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        println!(" part 2");
        timed_run(||{part2.resolve();});
        println!("    {}",part2);

    };
}

fn group_tuples<A,B>(mut input: Vec<(A,B)>) -> Vec<(A,Vec<B>)>
where A:std::cmp::Ord +  Copy,
B:std::cmp::Ord + Copy,
{
    input.sort();
    let mut ret = Vec::new();
    let mut cur;
    let mut it = input.iter();
    match it.next(){
        None => return ret,
        Some(x) => {
            ret.push( (x.0,Vec::new()));
            let len = ret.len()-1;
            cur = &mut ret[len];
        }
    }
    for i in input.iter(){
        if i.0 == cur.0 {
            cur.1.push(i.1);
        }else{
            ret.push( (i.0,Vec::new()));
            let len = ret.len()-1;
            cur = &mut ret[len];
        }
    }
    ret
}


fn timed_run<F,T>(mut f: F)->T where F: FnMut()->T{
    let start = Instant::now();
    let x = f();
    let duration = start.elapsed();
    println!("  Timing : {:?}", duration);
    x
}

fn main() -> Result<(), std::io::Error> {
    one_day!(day01);
    one_day!(day02);
    one_day!(day03);
    one_day!(day04);
    one_day!(day05);


    one_day!(day08);
    one_day!(day09);
    one_day!(day10);
    one_day!(day11);
    one_day!(day12);

    Ok(())  
}
