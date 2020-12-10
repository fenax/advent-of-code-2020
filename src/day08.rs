use super::*;
use super::puzzles::Data;
use std::vec::Vec;
use regex::Regex;
use itertools::Itertools;
use itertools::MinMaxResult::{NoElements, OneElement, MinMax};

#[derive(Clone)]
#[derive(Debug)]
#[derive(std::cmp::PartialEq)]
pub enum Instruction{
    Nop(i64),
    Acc(i64),
    Jmp(i64),
    End,
}

#[derive(Clone)]
pub struct Input{
    data: Vec<Instruction>
}

impl puzzles::Data<Instruction> for Input{
    fn new(input:&str) -> Input{
        let input = parser::one_string_per_line(input);
        let mut vector : Vec<Instruction> = input.iter().map(|x| {
                                    let i: Vec<&str>= x.split(' ').collect();
                                    let arg = i[1].parse::<i64>().unwrap();
                                    match i[0]{
                                        "acc" => Instruction::Acc(arg),
                                        "jmp" => Instruction::Jmp(arg),
                                        "nop" => Instruction::Nop(arg),
                                        _ => panic!("nooooooo")
                                    }}).collect();
        vector.push(Instruction::End);
        Input{ data : vector}
    }
    fn get_data(&self) -> &Vec<Instruction>{
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
                write!(f, "accumulator at end is {}", x)
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
        let code = self.data.get_data();
        let mut passed = vec![false;code.len()];
        let mut pc: usize = 0;
        let mut acc: i64 = 0;

        while(passed[pc] == false){
//            println!("pc {} ac {} {:?}",pc, acc, code[pc]);
            let mut next_pc = pc+1;
            passed[pc] = true;
            match code[pc]{
                Instruction::Acc(x) => {acc = acc + x},
                Instruction::Nop(_) => {},
                Instruction::Jmp(x) => {next_pc = (pc as i64 + x) as usize;}
                Instruction::End => {panic!("code should be wrong here.");}
            }
            pc = next_pc;
        }

        self.solution = Some(acc);
    }
}





impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let code = self.data.get_data();
        let mut passed = vec![false;code.len()];
        let mut pc: usize = 0;
        let mut acc: i64 = 0;

        let mut saved = None;

        while(code[pc] != Instruction::End){
            let mut skip = false;
            if passed[pc] {
                println!("restaure");
                skip = true;
                if let Some((npc,nacc,npassed)) = saved{
                    pc = npc;
                    acc = nacc;
                    passed = npassed;
                    saved = None;
                }
                else{
                    panic!("noooooooooo");
                }
            }
//            println!("pc {} ac {} {:?}",pc, acc, code[pc]);
            let mut next_pc = pc+1;
            passed[pc] = true;
            let mut inst = code[pc].clone();
            if saved.is_none() && !skip && inst != Instruction::Nop(0) {
                print!("Toggle {:?} at {}",inst,pc);
                match inst{
                    Instruction::Nop(x) => {saved = Some((pc,acc,passed.clone()));inst = Instruction::Jmp(x);},
                    Instruction::Jmp(x) => {saved = Some((pc,acc,passed.clone()));inst = Instruction::Nop(x);},
                    _ => {}
                }
                println!(" new instruction is {:?}",inst);
            }
            match inst
            {
                Instruction::Acc(x) => {acc = acc + x},
                Instruction::Nop(x) => {},
                Instruction::Jmp(x) => {next_pc = (pc as i64 + x) as usize;}
                Instruction::End => {self.solution = Some(acc); return;}
            }
            pc = next_pc;
            skip = false;
        }
        self.solution = Some(acc);
    }
}
