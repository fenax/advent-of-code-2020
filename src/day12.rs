use super::*;
use super::puzzles::Data;
use std::vec::Vec;


#[derive(Clone)]
#[derive(Debug)]
#[derive(std::cmp::PartialEq)]
pub enum Instruction{
    N(i64),
    E(i64),
    S(i64),
    W(i64),
    R(i64),
    L(i64),
    F(i64),
}

impl Instruction{
    /*fn get_data(&self)->i64{
        match self{
            Instruction::N(d) => *d,
            Instruction::E(d) => *d,
            Instruction::S(d) => *d,
            Instruction::W(d) => *d,
            Instruction::F(d) => *d,
            Instruction::R(d) => *d,
            Instruction::L(d) => *d,
        }
    }
    fn set_data(& mut self,data:i64){
        match self{
            Instruction::N(d) => *d = data,
            Instruction::E(d) => *d = data,
            Instruction::S(d) => *d = data,
            Instruction::W(d) => *d = data,
            Instruction::F(d) => *d = data,
            Instruction::R(d) => *d = data,
            Instruction::L(d) => *d = data,
        }
    }*/
    fn with_new_data(&self, data:i64)->Self{
        match self{
            Instruction::N(_) => Instruction::N(data),
            Instruction::E(_) => Instruction::E(data),
            Instruction::S(_) => Instruction::S(data),
            Instruction::W(_) => Instruction::W(data),
            Instruction::F(_) => Instruction::F(data),
            Instruction::R(_) => Instruction::R(data),
            Instruction::L(_) => Instruction::L(data),      
        }  
    }
}
#[derive(Clone)]
pub struct Input{
    data: Vec<Instruction>
}

impl puzzles::Data<Vec<Instruction>> for Input{
    fn new(input:&str) -> Input{
        let input = parser::one_string_per_line(input);
        let vector : Vec<Instruction> = input.iter().map(|x| {
                                    //let i: Vec<&str>= x.split(' ').collect();
                                    let mut c = x.chars();
                                    let com = c.next().unwrap();
                                    let arg = c.collect::<String>().parse::<i64>().unwrap();
                                    match com{
                                        'N' => Instruction::N(arg),
                                        'E' => Instruction::E(arg),
                                        'S' => Instruction::S(arg),
                                        'W' => Instruction::W(arg),
                                        'R' => Instruction::R(arg),
                                        'L' => Instruction::L(arg),
                                        'F' => Instruction::F(arg),
                                        _ => panic!("nooooooo")
                                    }}).collect();
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

fn rotate_by(instruction:&Instruction,direction:Instruction)-> Instruction{
    let mut right_angle = 
    match instruction{
        Instruction::R(a) => *a,
        Instruction::L(a) => -a,
        _ => panic!("should be right or left"),
    };
    println!("rotating by right {}",right_angle);
    while right_angle <  0   { right_angle += 360; }
    while right_angle >= 360 { right_angle -= 360; }
    let mut ret = direction.clone();
    while right_angle > 0{
        print!("R");
        right_angle -= 90;
        ret = 
        match ret{
            Instruction::N(_) => Instruction::E(0),
            Instruction::E(_) => Instruction::S(0),
            Instruction::S(_) => Instruction::W(0),
            Instruction::W(_) => Instruction::N(0),
            _ => panic!( "instruction is not a direction"),
        };
    }
    println!("  Now facing {:?}", ret);
    ret
}

fn move_by(instruction:&Instruction,(direction,(s,e)):(Instruction,(i64,i64)))->(Instruction,(i64,i64)){
    println!("{:?}  {:?}",instruction,(&direction,(s,e)));
    match instruction{
        Instruction::N(d) => (direction, (s-d,e)),
        Instruction::S(d) => (direction, (s+d,e)),
        Instruction::W(d) => (direction, (s, e-d)),
        Instruction::E(d) => (direction, (s, e+d)),
        Instruction::R(_d) => (rotate_by(instruction, direction),(s,e)),
        Instruction::L(_d) => (rotate_by(instruction, direction),(s,e)),
        Instruction::F(d) => {
            let dir2 = direction.with_new_data(*d);
            move_by(&dir2, (direction,(s,e)))
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

        let result = 
        code.iter().fold((Instruction::E(0),(0,0)),|(dir,(s,e)),instr|move_by(instr,(dir,(s,e))));

        self.solution = Some((result.1).0.abs() +(result.1).1.abs());
    }
}


fn rotate_by_two(instruction:&Instruction,(wp_s,wp_e):(i64,i64))-> (i64,i64){
    let mut right_angle = 
    match instruction{
        Instruction::R(a) => *a,
        Instruction::L(a) => -a,
        _ => panic!("should be right or left"),
    };
    println!("rotating by right {}",right_angle);
    while right_angle <  0   { right_angle += 360; }
    while right_angle >= 360 { right_angle -= 360; }

    let ret = 
        match right_angle{
            0   => (wp_s,wp_e),
            90  => (wp_e,-wp_s),
            180 => (-wp_s,-wp_e),
            270 => (-wp_e,wp_s),
            _ => panic!( "instruction is not a direction"),
        };
    println!("  Now facing {:?}", ret);
    ret
}

fn move_by_two(instruction:&Instruction,((wp_s, wp_e),(s,e)):((i64,i64),(i64,i64)))->((i64, i64),(i64,i64)){
    println!("{:?}  {:?}",instruction,((wp_s,wp_e),(s,e)));
    match instruction{
        Instruction::N(d) => ((wp_s - d, wp_e), (s,e)),
        Instruction::S(d) => ((wp_s + d, wp_e), (s,e)),
        Instruction::W(d) => ((wp_s, wp_e - d), (s, e)),
        Instruction::E(d) => ((wp_s, wp_e + d), (s, e)),
        Instruction::R(_) => (rotate_by_two(instruction, (wp_s, wp_e)),(s,e)),
        Instruction::L(_) => (rotate_by_two(instruction, (wp_s, wp_e)),(s,e)),
        Instruction::F(d) => {
            ((wp_s,wp_e),(s+wp_s*d,e+wp_e*d))
        }
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

        let result = 
        code.iter().fold(((-1,10),(0,0)),|(dir,(s,e)),instr|move_by_two(instr,(dir,(s,e))));

        self.solution = Some((result.1).0.abs() +(result.1).1.abs());
    }
}
