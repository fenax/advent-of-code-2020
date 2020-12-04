extern crate regex;

mod parser;
mod day01;
mod day02;
mod day03;
mod day04;
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

fn one_day<D,X,Y,T>(file:&str, title:&str) -> Result<(), std::io::Error>
where 
    D: puzzles::Data<T>,
    for<'a>  X: puzzles::Puzzle<'a,D>,
    for<'b>  Y: puzzles::Puzzle<'b,D> {
    let input = D::new(&read_to_string(file)?);
    {   
    let mut part1 = X::new(&input);
    let mut part2 = Y::new(&input);

    println!("{} part 1",title);
    timed_run(||{part1.resolve();});
    println!("{}",part1);

    println!("{} part 2",title);
    timed_run(||{part2.resolve();});
    println!("{}",part2);
    }    
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
 //   one_day::<day01::Input,day01::Part1,day01::Part2,_>("day01.input", "Day 01")?;
 //   one_day::<day02::Input,day02::Part1,day02::Part2,_>("day02.input", "Day 02")?;
//    one_day::<day03::Input,day03::Part1,day03::Part2,_>("day03.input", "Day 03")?;
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
        let input = day02::Input::new(&read_to_string("day02.input")?);
        let mut part1 = day02::Part1::new(&input);
        let mut part2 = day02::Part2::new(&input);

        println!("Day 02 part 1");
        timed_run(||{part1.resolve();});
        println!("{}",part1);

        println!("Day 02 part 2");
        timed_run(||{part2.resolve();});
        println!("{}",part2);       
    }
    {//Day 03
        let input = day03::Input::new(&read_to_string("day03.input")?);
        let mut part1 = day03::Part1::new(&input);
        let mut part2 = day03::Part2::new(&input);
//part2.learn();
        println!("Day 03 part 1");
        timed_run(||{part1.resolve();});
        println!("{}",part1);

        println!("Day 03 part 2");
        timed_run(||{part2.resolve();});
        println!("{}",part2);
    }  
    {//Day 03
        let input = day04::Input::new(&read_to_string("day04.input")?);
        let mut part1 = day04::Part1::new(&input);
        let mut part2 = day04::Part2::new(&input);
//part2.learn();
        println!("Day 04 part 1");
        timed_run(||{part1.resolve();});
        println!("{}",part1);
        println!("Day 04 part 2");
        timed_run(||{part2.resolve();});
        println!("{}",part2);
    }  Ok(()) }
