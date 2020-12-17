use super::*;
use super::puzzles::Data;
use std::collections::HashMap;


#[derive(Clone)]
pub struct Input{
    data: Vec<i64>
}

impl puzzles::Data<Vec<i64>> for Input{
    fn new(input:&str) -> Input{
        Input{ data : parser::coma_separated_int(input)}
    }
    fn get_data(&self) -> &Vec<i64>{
        &self.data
    }
}

pub struct Part1<'a>{
    data:&'a Input,
    solution: Option<usize>,
}
pub struct Part2<'a>{
    data:&'a Input,
    solution: Option<usize>,
}

impl<'a> std::fmt::Display for Part1<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some(x) => {
                write!(f, "{} is the last answer", x)
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
            Some(a) => {
                write!(f, "{}",a )
            }
            None => {
                write!(f, "no solution found")
            }
        }
    }
}

impl<'a> puzzles::Puzzle<'a, Input> for Part1<'a>
//where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part1{data:input,solution:None}
    }
    fn resolve(&mut self){
        let mut map:HashMap<usize,usize> = HashMap::new();
        let mut turn:usize = 0;
        let mut next:usize = 0;
        for i in self.data.get_data(){
            turn += 1;
            if let Some(v) = map.insert(*i as usize, turn){
                next = turn - v;
            }else{
                next = 0;
            }
        }
        for turn in turn+1..2020{
            if let Some(v) = map.insert(next, turn){
                next = turn - v;
            }else{
                next = 0;
            }
        }
        self.solution = Some(next);
    }
}

impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let mut map:HashMap<usize,usize> = HashMap::new();
        let mut turn:usize = 0;
        let mut next:usize = 0;
        for i in self.data.get_data(){
            turn += 1;
            if let Some(v) = map.insert(*i as usize, turn){
                next = turn - v;
            }else{
                next = 0;
            }
        }
        for turn in turn+1..30000000{
            if let Some(v) = map.insert(next, turn){
                next = turn - v;
            }else{
                next = 0;
            }
        }
        self.solution = Some(next);
    }
}
