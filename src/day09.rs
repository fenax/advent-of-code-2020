use super::*;
use super::puzzles::Data;
use std::vec::Vec;

#[derive(Clone)]
pub struct Input{
    data: Vec<i64>
}

impl puzzles::Data<i64> for Input{
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
                write!(f, "Number that does not follow the rule is {}", x)
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
                write!(f, "accumulator at end is {}", x)
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
        let input = self.data.get_data();
        for i in 25..input.len(){
            let mut slice = input[i-25..i].to_vec();
            slice.sort();
            if let None = day01::find_with_sum(&slice,input[i]){
                self.solution = Some(input[i]);
                return
            }
        }
    }
}




impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){

    }
}
