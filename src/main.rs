mod parser;
mod day01;

use std::fs::read_to_string;


fn main() -> Result<(), std::io::Error> {
    {//Day 01
        let mut input_day01 = parser::one_int_per_line(&read_to_string("day01.input")?);
        input_day01.sort();
        day01::find_with_sum(&input_day01, 2020);
        day01::find_three_with_sum(&input_day01, 2020);
    }
    println!("Hello, world!");
    Ok(())
}
