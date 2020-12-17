use super::*;
use std::vec::Vec;
use std::collections::HashMap;


#[derive(Clone)]
#[derive(Debug)]
pub enum Thing{
    Mask{
        or_mask:u64,
        and_mask:u64,
        bits:Vec<char>,
    },
    Mem{
        address:u64,
        value:u64,
    }
}

#[derive(Clone)]
pub struct Input{
    data: Vec<Thing>
}

fn parse_mask(i:&str)->Thing{
    let mut and = 0;
    let mut or = 0;
    for c in i.chars(){
        match c {
            '1' => {
                or  = or * 2 + 1;
                and = and * 2 + 1;
            },
            '0' => {
                or  = or * 2;
                and = and * 2;
            },
            'X' => {
                or  = or * 2;
                and = and * 2 + 1;
            },
            _ => panic!("unknown character"),
        }
    }
    Thing::Mask{or_mask:or, and_mask:and,bits:i.chars().collect()}
}
/*        1111111111222222222233333333334444444444
01234567890123456789012345678901234567890123456789
mask = 0X0X1110X1010X1X10010X0011010X100110
mem[40190] = 23031023
*/
fn parse_line(i:&String)->Thing{
    if i.starts_with("mask"){
        parse_mask(&i[7..])
    }else if i.starts_with("mem"){
        let address = i[4..].chars().take_while(|x| *x!=']').collect::<String>().parse::<u64>().unwrap();
        let value = i[i.rfind(' ').unwrap()+1..].parse::<u64>().unwrap();
        Thing::Mem{address:address,value:value}
    }else{
        panic!("bad input");
    }
}

impl puzzles::Data<Vec<Thing>> for Input{
    fn new(input:&str) -> Input{
        let input = parser::one_string_per_line(input);
        let vector = input.iter().map(parse_line).collect();
        Input{ data : vector}
    }
    fn get_data(&self) -> &Vec<Thing>{
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
                write!(f, "{} is the solution", x)
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
                write!(f, "{} is the solution", x)
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
        let mut memory:HashMap<u64,u64> = HashMap::new();
        let mut and_mask:u64 = 0xFFFFFFFFF;
        let mut or_mask:u64  = 0;
        for x in self.data.get_data(){
            println!("{:?}",x);
            match x{
                Thing::Mask{or_mask:or,and_mask:and,bits:_} =>{
                    and_mask = *and;
                    or_mask = *or;
                },
                Thing::Mem{address,value} =>{
                    let v = value & and_mask;
                    let w = v | or_mask;
                    memory.insert(*address, w);
                }
            }
        }
        self.solution = Some(memory.iter().map(|(k,v)|*v).sum());
    }
}

impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let mut memory:HashMap<u64,u64> = HashMap::new();
        let mut or_mask:u64  = 0;
        let mut float_bits:Vec<usize> = vec![];
        for x in self.data.get_data(){
            match x{
                Thing::Mask{or_mask:or,and_mask:_,bits:b} =>{
                    float_bits.clear();
                    for (i,bit) in b.iter().enumerate(){
                        if *bit == 'X' {
                            float_bits.push(35-i);
                        }
                    }
                    or_mask = *or;
                },
                Thing::Mem{address,value} =>{
                    let w = address | or_mask;
                    let mut addresses = vec![w;2_usize.pow( float_bits.len() as u32)];
                    for (i,shift) in float_bits.iter().enumerate(){
                        let base_mask:u64 = 0xFFFFFFFFF;
                        let or_mask:u64 = 1<<shift;
                        let and_mask = base_mask^or_mask;
                        let step_size = 1<<i;
                        for j in 0..step_size{
                            let original = addresses[j];
                            addresses[j] = original & and_mask;
                            addresses[j+step_size] = original | or_mask;
                        }
                    }
                    for address in addresses{
                        memory.insert(address, *value);
                    }
                }
            }
        }
        self.solution = Some(memory.iter().map(|(k,v)|*v).sum());
    }
}

