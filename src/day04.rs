use super::*;
use super::puzzles::Data;
use std::vec::Vec;
use regex::Regex;

#[derive(Clone)]
pub struct Input{
    data: Vec<Vec<String>>
}

impl puzzles::Data<Vec<Vec<String>>> for Input{
    fn new(input:&str) -> Input{
        Input{ data : parser::items_separated_by_whitespace_separated_by_blankline(input)}
    }
    fn get_data(&self) -> &Vec<Vec<String>>{
        &self.data
    }
}

pub struct Part1<'a>{
    data:&'a Input,
    solution: Option<u64>,
}
pub struct Part2<'a>{
    data:&'a Input,
    solution: Option<u64>,
}

impl<'a> std::fmt::Display for Part1<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some(x) => {
                write!(f, "{} valid passports", x)
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
                write!(f, "{} valid passports", x)
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
        let fields = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];
        self.solution = Some(count_valid_passports(self.data.get_data(), &fields));
    }
}

fn count_valid_passports(input:&Vec<Vec<String>>, fields:&Vec<&str>)->u64{
    let mut count = 0; 
    'outer:for l in input.iter(){
        for i in fields.iter(){
            match l.iter().find(|x| x.as_str().starts_with(i)){
                Some(_) => {},
                None => {continue 'outer;}
            }
        }
        count = count + 1;
    }
    count
} 

impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let input = self.data.get_data();
        let mut count = 0; 
        let re_byr = Regex::new(r"byr:(\d{4})").unwrap();
        let re_iyr = Regex::new(r"iyr:(\d{4})").unwrap();
        let re_eyr = Regex::new(r"eyr:(\d{4})").unwrap();
        let re_hgt = Regex::new(r"hgt:(\d{2,3})([a-z]{2})").unwrap();
        let re_hcl = Regex::new(r"hcl:#([[:xdigit:]]{6})").unwrap();
        let re_ecl = Regex::new(r"ecl:([a-z]{3})").unwrap();
        let re_pid = Regex::new(r"pid:([0-9]{9})[^0-9]").unwrap();
        let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];


        for l in input.iter(){
            let mut full = l.join(" ");
            full.push(' ');
            //println!("{}-{:?}",full, (
            let res = vec![ re_byr.captures_iter(&full).map(|x| { x[1].parse::<u32>().unwrap_or(0)}).any(|v|  v>=1920 && v<=2002)
            , re_iyr.captures_iter(&full).map(|x| { x[1].parse::<u32>().unwrap_or(0)}).any(|v|  v>=2010 && v<=2020)
            , re_eyr.captures_iter(&full).map(|x| { x[1].parse::<u32>().unwrap_or(0)}).any(|v|  v>=2020 && v<=2030)
            , re_hgt.captures_iter(&full).map(|x| { (x[1].parse::<u32>().unwrap_or(0), x[2].to_string())
                }).any(|(v,u)| {(u=="cm" && v>=150 && v<=193) || (u=="in" && v>=59 && v<=76)})
            , re_hcl.captures_iter(&full).any(|_| true)
            , re_ecl.captures_iter(&full).any(|x| eye_colors.iter().any(|&y| x[1]==*y))
            , re_pid.captures_iter(&full).any(|_| true)];
//            println!("{}-{:?}",full,res);
            if res.iter().all(|x|*x)
            {
                count = count + 1;
            }
        }        
        self.solution = Some(count);
    }
}