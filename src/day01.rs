use super::*;
use super::puzzles::Data;

pub struct Input{
    data: Vec<i64>
}

impl puzzles::Data<i64> for Input{
    fn new(input:&str) -> Input{
        let d = parser::one_int_per_line(&input);
        Input{ data : d}
    }
    fn get_data(&self) -> &Vec<i64>{
        &self.data
    }
}

pub struct Part1{
    data: Vec<i64>,
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

impl puzzles::Puzzle<'_,Input> for Part1{
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

impl puzzles::Puzzle<'_, Input> for Part2{
    fn new(input:&Input)->Self{
        let mut v = input.get_data().clone();
        v.sort();
        Part2{data:v,solution:None}
    }
    fn resolve(&mut self){
        self.solution = find_three_with_sum(&self.data, 2020);
    }
}


pub fn find_with_sum_exclude(input:&Vec<i64>,target:i64,exclusion_start:usize,exclusion_stop:usize)->Option<(usize,usize)>{
        let mut i = 0; let mut j = input.len()-1;
        while i<j && i<exclusion_start && exclusion_stop<j{
            let sum = input[i] + input[j];
            if sum == target {return Some((i,j))}
            else if sum > target {j = j-1; }
            else if sum < target {i = i+1;}
        }
        None
}

pub fn find_three_with_sum(input:&Vec<i64>,target:i64)->Option<(usize,usize,usize)>{
    let mut k = input.iter().position(|x| *x >= target/3).unwrap_or(input.len()-2);
    let mut l = k;
    let mut cur = &k;
    let mut dec = true; //first turn decrement k, next increment l, then start again
    while k>1 || l<input.len()-2{
        if let Some((i,j)) = find_with_sum_exclude(input,target-input[*cur],k,l){
            return Some((i,j,*cur))
        }else{
            if (dec && k>2) || l>= input.len()-2{
                k = k-1;
                cur = &k;
                dec = false;
            }else{
                l = l+1;
                cur = &l;
                dec = true;
            }
        }
    }
    None
}