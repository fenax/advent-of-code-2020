use super::*;
use super::puzzles::Data;
use std::vec::Vec;


#[derive(Clone)]
pub struct Input{
    data: (i64,Vec<Option<i64>>)
}

impl puzzles::Data<(i64,Vec<Option<i64>>)> for Input{
    fn new(input:&str) -> Input{
        let input = parser::one_string_per_line(input);
        let stamp = input[0].parse::<i64>().expect("should be int");
        let v = input[1].split(',').map(|x|{
            if x == "x"{
                None
            }else{
                x.parse::<i64>().ok()
            }
        }).collect();
        Input{ data : (stamp,v)}
    }
    fn get_data(&self) -> &(i64,Vec<Option<i64>>){
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
                write!(f, "Solution is {}", x)
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
                write!(f, "Solution is {}", x)
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
        let (stamp, list) = self.data.get_data();
        let closest = list.iter().filter_map(|x|*x).map(|x|{
            let s = stamp - (stamp%x);
            if s == *stamp{
                (s,x)
            }else{
                (s + x,x)
            }
        }).min().unwrap();
        self.solution =  Some((closest.0 -stamp) * closest.1);
    }
}





impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let (_,list) = self.data.get_data();
        let mut passage:Vec<(i64,usize)> = vec![(1,0);list.len()];
        for (i,bus) in list.iter().enumerate(){
            passage[i].1 = i;
            if let Some(bus) = bus{
                let mut j = i as i64-bus;
                passage[i].0 *= bus;
                while j>=0{
                    passage[j as usize].0 *= bus;
                    j -= bus;
                }
                let mut j = i as i64+bus;
                while j<passage.len() as i64{
                    passage[j as usize].0 *= bus;
                    j += bus;
                }
            }
        }
        passage.sort();
        let (largest_bus, lagrest_bus_pos) = passage.pop().unwrap();
        passage.reverse();
        let mut current_point = 0;
        'outer: loop{
            current_point += largest_bus;
            let at_first = current_point - lagrest_bus_pos as i64;
                if passage.iter().all(|(bus,pos)| (*pos as i64+at_first)%bus == 0){
                    break 'outer;
                }
            
        }
        self.solution = Some(current_point - lagrest_bus_pos as i64);
    }
}