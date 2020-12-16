use super::*;
use super::puzzles::Data;
use regex::Regex;


#[derive(Clone)]
pub struct Input{
    data: Vec<(u8,u8,char,String)>
}

impl puzzles::Data<Vec<(u8,u8,char,String)>> for Input{
    fn new(input:&str) -> Input{
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]).{2}([a-z]*)").unwrap();
        Input{ data : re.captures_iter(input).map(|x| (
                            x[1].parse::<u8>().unwrap(),
                            x[2].parse::<u8>().unwrap(),x[3].chars().nth(0).unwrap_or('0'),
                            x[4].to_string())).collect()}
    }
    fn get_data(&self) -> &Vec<(u8,u8,char,String)>{
        &self.data
    }
}

pub struct Part1{
    data: Input,
    solution: Option<(usize,usize)>,
}
pub struct Part2{
    data: Input,
    solution: Option<(usize,usize)>,
}

impl std::fmt::Display for Part1{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some((i,j)) => {
                write!(f, "{} valid, {} invalid", i, j)
            }
            None => {
                write!(f, "no solution found")
            }
        }
    }
}
 
impl std::fmt::Display for Part2{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some((i,j)) => {
                write!(f, "{} valid, {} invalid", i, j)
            }
            None => {
                write!(f, "no solution found")
            }
        }
    }
}
 
fn is_valid(data:&(u8,u8,char,String)) ->bool{
    let (min, max, c, src) = data;
    let mut i = 0;
    for x in src.chars(){
        if *c == x {i=i+1;}
    }
    i<=*max && i>=*min
}
fn is_valid_2(data:&(u8,u8,char,String)) ->bool{
    let (ia, ib, c, src) = data;
    let a = src.chars().nth((*ia-1)as usize).unwrap_or('0');
    let b = src.chars().nth((*ib-1)as usize).unwrap_or('0');
    (a==*c || b == *c) && a!=b 
}

impl puzzles::Puzzle<'_,Input> for Part1{
    fn new(input:&Input)->Self{
        let v = input.clone();
        Part1{data:v,solution:None}
    }
    fn resolve(&mut self){
        let mut t = 0;
        let mut f = 0;
        for x in self.data.get_data().iter().map(is_valid)
        {
            if x {t=t+1;}else{f=f+1}
        }
        self.solution = Some((t,f))
    }
}

impl puzzles::Puzzle<'_,Input> for Part2{
    fn new(input:&Input)->Self{
        let v = input.clone();
        Part2{data:v,solution:None}
    }
    fn resolve(&mut self){
        let mut t = 0;
        let mut f = 0;
        for x in self.data.get_data().iter().map(is_valid_2)
        {
            if x {t=t+1;}else{f=f+1}
        }
        self.solution = Some((t,f))
    }
}

