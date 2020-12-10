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
mod puzzles;

use std::fs::read_to_string;
use std::time::Instant;
use puzzles::*;
use std::vec::Vec;

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
/*
fn one_part<'data,'b,D,X,T>(part_number:&str,data:& D)
where 
    D: puzzles::Data<T>,
    for<'a> X: puzzles::Puzzle<'data, D> +'a,
    X: 'b,
    'data:'b
{
    let mut part = X::new(data);
    println!("  part {}",part_number);
    timed_run(||{part.resolve();});
    println!("{}",part);  
}

fn do_with_data<F,T> (data:&T,mut f:F) where F: FnMut(&T){
    f(data);
}

fn one_day<'data,D,X,Y,T>(file:&str, title:&str) -> Result<(), std::io::Error>
where 
    D: puzzles::Data<T> + 'data,
    X: puzzles::Puzzle<'data, D>,
    Y: puzzles::Puzzle<'data, D>,
    for<'a> X:'a,
    for<'b> Y:'b {
    let input = D::new(&read_to_string(file)?);
    {   
        one_part::<D,X,T>("1",&input);
        one_part::<D,Y,T>("2",&input);
    }    
    Ok(())
}*/
/*
fn one_day<'a,'b,'d,D,X,Y,T>(file:&str, title:&str, ) -> Result<(), std::io::Error>
where 
    D: puzzles::Data<T> +'d,
    X: puzzles::Puzzle<'a,D>,
    Y: puzzles::Puzzle<'b,D>,
    'd : 'a + 'b,
{
    let input = D::new(&read_to_string(file)?);
    {   
    let mut part1 = X::new(& input);
    let mut part2 = Y::new(& input);

    println!("{} part 1",title);
    timed_run(||{part1.resolve();});
    println!("{}",part1);

    println!("{} part 2",title);
    timed_run(||{part2.resolve();});
    println!("{}",part2);
    }    
    Ok(())
}
*/
fn main() -> Result<(), std::io::Error> {
 //  one_day::<day01::Input,day01::Part1,day01::Part2,_>("day01.input", "Day 01")?;
 //   one_day::<day02::Input,day02::Part1,day02::Part2,_>("day02.input", "Day 02")?;
 //   one_day::<day03::Input,day03::Part1,day03::Part2,_>("day03.input", "Day 03")?;
   {//Day 01
        println!("Day 01");
        let i_string = read_to_string("day01.input")?;
        let input =timed_run(||{day01::Input::new(&i_string)});
       
        let mut part1 = day01::Part1::new(&input);
        let mut part2 = day01::Part2::new(&input);

        println!(" part 1");
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        println!(" part 2");
        timed_run(||{part2.resolve();});
        println!("    {}",part2);

    }
    {//Day 02
        println!("Day 02");
        let i_string = read_to_string("day02.input")?;
        let input =timed_run(||{day02::Input::new(&i_string)});

        let mut part1 = day02::Part1::new(&input);
        let mut part2 = day02::Part2::new(&input);

        println!(" part 1");
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        println!(" part 2");
        timed_run(||{part2.resolve();});
        println!("    {}",part2);   
    }
    {//Day 03
        println!("Day 03");
        let i_string = read_to_string("day03.input")?;
        let input =timed_run(||{day03::Input::new(&i_string)});
        
        let mut part1 = day03::Part1::new(&input);
        let mut part2 = day03::Part2::new(&input);

        println!(" part 1");
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        println!(" part 2");
        timed_run(||{part2.resolve();});
        println!("    {}",part2);   
    } 
    {//Day 04
        println!("Day 04");
        let i_string = read_to_string("day04.input")?;
        let input =timed_run(||{day04::Input::new(&i_string)});

        let mut part1 = day04::Part1::new(&input);
        let mut part2 = day04::Part2::new(&input);

        println!(" part 1");
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        println!(" part 2");
        timed_run(||{part2.resolve();});
        println!("    {}",part2);   
    } 
    {//Day 05
        println!("Day 05");
        let i_string = read_to_string("day05.input")?;
        let input =timed_run(||{day05::Input::new(&i_string)});

        let mut part1 = day05::Part1::new(&input);
        let mut part2 = day05::Part2::new(&input);

        println!(" part 1");
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        println!(" part 2");
        timed_run(||{part2.resolve();});
        println!("    {}",part2);
    }   
      {//Day 08
        println!("Day 08");
        let i_string = read_to_string("day08.input")?;
        let input =timed_run(||{day08::Input::new(&i_string)});

        let mut part1 = day08::Part1::new(&input);
        let mut part2 = day08::Part2::new(&input);

        println!(" part 1");
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        println!(" part 2");
        timed_run(||{part2.resolve();});
        println!("    {}",part2);
    } 
    {//Day 09
        println!("Day 09");
        let i_string = read_to_string("day09.input")?;
        let input =timed_run(||{day09::Input::new(&i_string)});

        let mut part1 = day09::Part1::new(&input);
        let mut part2 = day09::Part2::new(&input);

        println!(" part 1");
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        println!(" part 2");
        timed_run(||{part2.resolve();});
        println!("    {}",part2);
    }       Ok(())  
}
