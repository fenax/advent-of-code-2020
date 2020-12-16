use super::*;
use super::puzzles::Data;
use std::vec::Vec;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Input{
    data: Vec<Vec<String>>
}

impl puzzles::Data<Vec<String>> for Input{
    fn new(input:&str) -> Input{
        Input{ data : parser::items_separated_by_whitespace_separated_by_blankline(input)}
    }
    fn get_data(&self) -> &Vec<Vec<String>>{
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
                write!(f, "{} yes replies", x)
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
                write!(f, "{} yes replies", x)
            }
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
        let mut list:Vec<Vec<char>> = self.data.get_data().iter().map(|x|x.concat().chars().collect()).collect();
        let list = list.iter().map(|x|{let mut x = x.to_owned();x.sort();x.dedup();x.len() as i64}).sum();

        self.solution = Some(list);
    }
}


impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        self.solution = Some(self.data.get_data().iter().map(
            |i|{
                let mut iterator = i.iter();
                let first = iterator.next().unwrap();
                let rest : Vec<HashSet<char>> = iterator.map(|l|l.chars().collect()).collect();
                first.chars().filter(|k| rest.iter().all(|s| s.contains(k))).count() as i64
            }).sum());

    }
}