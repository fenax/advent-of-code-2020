use super::*;
use super::puzzles::Data;
use std::vec::Vec;


#[derive(Clone)]
pub struct Input{
    data: Vec<i64>
}

impl puzzles::Data<Vec<i64>> for Input{
    fn new(input:&str) -> Input{
        let input = parser::one_int_per_line(input);
        Input{ data : input}
    }
    fn get_data(&self) -> &Vec<i64>{
        &self.data
    }
}

pub struct Part1<'a>{
    data:&'a Input,
    solution: Option<i64>,
}
pub struct Part2<'a>{
    data:&'a Input,
    solution: Option<i64>,
}

impl<'a> std::fmt::Display for Part1<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some(x) => {
                write!(f, "Solution is {}", x)
            }
            None => {
                write!(f, "no solution found")
            }
        }
    }
}
impl<'a> std::fmt::Display for Part2<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some(x) => {
                write!(f, "Solution is {}", x)
            },
            None => {
                write!(f, "no solution found")
            }
        }
    }
}


impl<'a,'b> puzzles::Puzzle<'a, Input> for Part1<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part1{data:input,solution:None}
    }
    fn resolve(&mut self){
        let mut input = self.data.get_data().clone();
        input.sort();
        let mut last = 0;
        let mut one = 0;
        let mut three = 1;// because the device integrated adapter
        for i in input{
            match i-last{
                1 => one = one + 1,
                3 => three = three + 1,
                _ => println!("should this happen ?"),
            }
            last = i;
        }
        self.solution = Some(one * three);
    }
}




impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let mut input = self.data.get_data().clone();
        input.sort();
        let mut last = 0;
        let mut one = 0;
        let mut sum = 1;
        //let mut three = 1;// because the device integrated adapter
        for i in input{
            match i-last{
                1 => one = one + 1,
                3 => {
                    match one{
                        3 => sum = sum*4,
                        4 => sum = sum*7,
                        2 => sum = sum*2,
                        0 => {},
                        _ => panic!("squee")
                    }
                    //println!("{} ones in a row", one);
                    one = 0;
                },

                _ => println!("should this happen ?"),
            }
            last = i;
        }
        self.solution = Some(sum);
    }
}
/*
1 0 0 1 
1 1 0 1
1 0 1 0
1 1 1 1


non 1 0 0 0 1
1 1 1 1 1
1 0 0 1 1
1 0 1 0 1
1 1 0 0 1
1 1 1 0 1
1 1 0 1 1
1 0 1 1 1


1 0 1
1 1 1*/