use super::*;
use super::puzzles::Data;
use std::vec::Vec;
use regex::Regex;
use itertools::Itertools;
use itertools::MinMaxResult::{NoElements, OneElement, MinMax};

#[derive(Clone)]
pub struct Input{
    data: Vec<(u32,u32)>
}

fn parse_boarding_pass(i:&String)->(u32,u32){
    let mut row = 0;
    let mut seat = 0;
    for c in i.chars(){
        match c {
            'F' => row = row*2,
            'B' => row = row*2 +1,
            'L' => seat = seat*2,
            'R' => seat = seat*2+1,
            _ => panic!("unknown character"),
        }
    }
    (row,seat)
}

impl puzzles::Data<(u32,u32)> for Input{
    fn new(input:&str) -> Input{
        let input = parser::one_string_per_line(input);
        let vector = input.iter().map(parse_boarding_pass).collect();
        Input{ data : vector}
    }
    fn get_data(&self) -> &Vec<(u32,u32)>{
        &self.data
    }
}

pub struct Part1<'a>{
    data:&'a Input,
    solution: Option<u32>,
}
pub struct Part2<'a>{
    data:&'a Input,
    solution: Option<(u32,u32,u32)>,
}

impl<'a> std::fmt::Display for Part1<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some(x) => {
                write!(f, "highest seat ID is {}", x)
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
            Some((x,y,z)) => {
                write!(f, "your seat is {} {} seat id {}", x,y,z)
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
        self.solution = self.data.get_data().iter().map(|(a,b)| a*8+b).max();
    }
}

impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let d = self.data.get_data();
        if let MinMax(min,max) = d.iter().map(|(a,_b)|a).minmax(){
            let mut v : Vec<Vec<bool>> = vec![vec![false; 8]; (max - min + 1) as usize];
            for i in d.iter(){
                v[(i.0-min) as usize][i.1 as usize] = true;
            }
            //println!("{:?}",v)
            for (row, sv) in v.iter().enumerate(){
                for (seat, i) in sv.iter().enumerate(){
                    if !i {
                        self.solution = Some((row as u32+min,seat as u32,(row as u32+min)*8+seat as u32));
                        return;
                    }
                }
            }
        }
    }
}

