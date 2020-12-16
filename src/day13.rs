use super::*;
use super::puzzles::Data;
use std::vec::Vec;


#[derive(Clone)]
pub struct Input{
    data: (i64,Vec<Option<i64>>)
}

impl puzzles::Data<(i64,Vec<Option<i64>>)> for Input{
    fn new(input:&str) -> Input{
        let input = parser::one_string_per_line(input);
        let stamp = input[0].parse::<i64>().expect("should be int");
        let v = input[1].split(',').map(|x|{
            if x == "x"{
                None
            }else{
                x.parse::<i64>().ok()
            }
        }).collect();
        Input{ data : (stamp,v)}
    }
    fn get_data(&self) -> &(i64,Vec<Option<i64>>){
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