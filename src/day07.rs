use super::*;
use super::puzzles::Data;
use std::vec::Vec;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Input{
    names: Vec<(String,String)>,
    data: Vec<(usize,Vec<(i64,usize)>)>
}

impl puzzles::Data<Vec<(usize,Vec<(i64,usize)>)>> for Input{
    fn new(input:&str) -> Input{
        let lines = parser::one_string_per_line(input);
        let mut names:Vec<(String,String)> = lines.iter().map(|l|{
            let line:Vec<_> = l.splitn(3," ").collect();
            (line[0].to_string(),line[1].to_string())
        }).collect();
        names.sort();
        let v = lines.iter().map(|line|{
            let line:Vec<_> = line.split(" ").collect();
            let len = line.len();
            (names.binary_search(&(line[0].to_string(),line[1].to_string())).expect("bad pragrammer"),
                if len == 7{//contains no other bag
                    vec![]
                }else if len%4 == 0{
                    let count = len/4;
                    (1..count).map(|i|{
                        (line[i*4].parse::<i64>().unwrap(),names.binary_search(&(line[i*4+1].to_string(),line[i*4+2].to_string())).expect("still bad programer"))
                    }).collect()
                }else{
                    panic!("invalid data");
                })
        }).collect();
        Input{ data : v,names:names}
    }
    fn get_data(&self) -> &Vec<(usize,Vec<(i64,usize)>)>{
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
                write!(f, "{} different bags can countain gold", x)
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
                write!(f, "{} bags must be countained within the gold bag", x)
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
        let data = self.data.get_data();
        let mut contained:Vec<Vec<usize>> = vec![vec![];data.len()];
        for (jar, jam) in data.iter(){
            for (_,fruit) in jam{
                contained[*fruit].push(*jar);
            }
        }
        let gold = self.data.names.binary_search(&("shiny".to_string(),"gold".to_string())).expect("no gold, no game");
        let mut can_contain_gold:HashSet<usize> = contained[gold].iter().map(|x| *x).collect();

        loop{
            let new_contain_gold:HashSet<usize> = can_contain_gold.iter().map(|x|{
                contained[*x].clone()
            }).flatten().collect();
            if new_contain_gold.difference(&can_contain_gold).count() > 0{
                can_contain_gold = can_contain_gold.union(&new_contain_gold).map(|x|*x).collect();
            }else{
                break;
            }
            self.solution = Some(can_contain_gold.len()as i64);
        }
    }
}


fn count_bags(data: &Vec<(usize,Vec<(i64,usize)>)>, x:usize)->i64{
    data[x].1.iter().map(|(count,y)|{
        count_bags(data, *y) * count
    }).sum::<i64>() + 1
}

impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let mut data = self.data.get_data().clone();
        data.sort();

        let gold = self.data.names.binary_search(&("shiny".to_string(),"gold".to_string())).expect("no gold, no game");

        self.solution = Some(count_bags(&data, gold) - 1); // -1 because the function count the bag itself
    }
}