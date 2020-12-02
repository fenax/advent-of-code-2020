use super::*;
use super::puzzles::Data;

pub struct Input{
    data: Vec<(u8,u8,char,String)>
}

impl puzzles::Data<i64> for Input{
    fn new(input:&str) -> Input{
        let re = Regex::new(r"(\d{1-2})-(\d{1-2})-([a-z]): ([a-z]*)").unwrap();
        re.captures_iter(input).map(|x| (x[1],x[2],x[3],x[4])).collect();
        Input{ data : re.captures_iter(input).map(|x| (x[1],x[2],x[3],x[4])).collect()}
    }
    fn get_data(&self) -> &Vec<i64>{
        &self.data
    }
}

pub struct Part1{
    data: Input,
    solution: Option<(usize,usize)>,
}

impl std::fmt::Display for Part1{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some((i,j)) => {
                let di = self.data[i];
                let dj = self.data[j];
                write!(f, "elements [{}]{} and [{}]{} -> {}", i, di, j, dj, di * dj)
            }
            None => {
                write!(f, "no solution found")
            }
        }
    }
}
 
fn is_valid(data:(u8,u8,char,String)) ->bool{
    let (min, max, c, src) = data;

}


impl puzzles::Puzzle<Input> for Part1{
    fn new(input:&Input)->Self{
        let mut v = input.get_data().clone();
        v.sort();
        Part1{data:v,solution:None}
    }
    fn resolve(&mut self){
        self.solution = find_with_sum_exclude(&self.data,2020,self.data.len(),0);
    }
}

pub struct Part2{
    data: Vec<i64>,
    solution: Option<(usize,usize,usize)>,
}

impl std::fmt::Display for Part2{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some((i,j,k)) => {
                let di = self.data[i];
                let dj = self.data[j];
                let dk = self.data[k];
                write!(f, "elements [{}]{}, [{}]{} and [{}]{} -> {}", i, di, j, dj, k, dk, di * dj * dk)
            }
            None => {
                write!(f, "no solution found")
            }
        }
    }
}

impl puzzles::Puzzle<Input> for Part2{
    fn new(input:&Input)->Self{
        let mut v = input.get_data().clone();
        v.sort();
        Part2{data:v,solution:None}
    }
    fn resolve(&mut self){
        self.solution = find_three_with_sum(&self.data, 2020);
    }
}

