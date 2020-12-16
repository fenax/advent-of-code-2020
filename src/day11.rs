use super::*;
use super::puzzles::Data;
use std::vec::Vec;


#[derive(Clone)]
pub struct Input{
    data: Vec<Vec<char>>
}

impl puzzles::Data<Vec<Vec<char>>> for Input{
    fn new(input:&str) -> Input{
        let input = parser::one_char_vec_per_line(input);
        Input{ data : input}
    }
    fn get_data(&self) -> &Vec<Vec<char>>{
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

#[allow(non_upper_case_globals)]
const directions:[(isize,isize);8] = [(-1,-1),(-1,0),(-1,1),
                    ( 0,-1),       ( 0,1),
                    ( 1,-1),( 1,0),( 1,1)];

fn step_map(map:&Vec<Vec<char>>,occupation:& mut Vec<Vec<bool>>)->i64{
    let old_occupation = occupation.clone();
    let mut changes = 0;
    let count_surrounding = |x: isize, y: isize|{
        let width = map[0].len() as isize;
        let height = map.len()   as isize;
        let read_map = |x:isize,y:isize|->bool{
            if x >= 0 && x < width && y >=0 && y < height{
                map[y as usize][x as usize] == 'L' && old_occupation[y as usize][x as usize]
            }else{
                false
            }
        };
        let v : Vec<bool> = directions.iter().map(|(y_of,x_of)| read_map(x+x_of,y+y_of)).collect();
        v.iter().filter(|&&x| x).count() as i64
    };
    for y in 0..map.len(){
        for x in 0..map[0].len(){
            if map[y][x] == 'L'{
                let count = count_surrounding(x as isize, y as isize);
                if count == 0{
                    if occupation[y][x] == false {
                        changes += 1;
                        occupation[y][x] = true;
                    }
                }else if count >= 4{
                    if occupation[y][x] == true{
                        changes += 1;
                        occupation[y][x] = false;
                    }
                }
            }
        }
    }
    changes
}


impl<'a,'b> puzzles::Puzzle<'a, Input> for Part1<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part1{data:input,solution:None}
    }
    fn resolve(&mut self){
        let map = self.data.get_data();
        let mut occupation = vec![vec![false;map[0].len()];map.len()];
        loop{
            let val = step_map(map,& mut occupation);
//            println!("{} changes", val);
            if val == 0 {break;}
        }
        self.solution = Some(occupation.iter().map(|x| x.iter().filter(|&&y| y).count() as i64).sum())
    }
}



fn step_map_two(map:&Vec<Vec<char>>,occupation:& mut Vec<Vec<bool>>)->i64{
    let old_occupation = occupation.clone();
    let mut changes = 0;
    let count_surrounding = |x: isize, y: isize|{
        let width = map[0].len() as isize;
        let height = map.len()   as isize;
        let read_map = |x:isize,x_of:isize,y:isize,y_of:isize|->bool{
            let mut dist = 1;
            loop {
                let x = x+x_of*dist;
                let y = y+y_of*dist;
                if x >= 0 && x < width && y >=0 && y < height{
                    if map[y as usize][x as usize] == 'L' {
                        return old_occupation[y as usize][x as usize];
                    }else{
                        dist += 1;
                        continue;
                    }
                }else{
                    return false;
                }
            }
        };
        let v : Vec<bool> = directions.iter().map(|(y_of,x_of)| read_map(x,*x_of,y,*y_of)).collect();
        v.iter().filter(|&&x| x).count() as i64
    };
    for y in 0..map.len(){
        for x in 0..map[0].len(){
            if map[y][x] == 'L'{
                let count = count_surrounding(x as isize, y as isize);
                if count == 0{
                    if occupation[y][x] == false {
                        changes += 1;
                        occupation[y][x] = true;
                    }
                }else if count >= 5{
                    if occupation[y][x] == true{
                        changes += 1;
                        occupation[y][x] = false;
                    }
                }
            }
        }
    }
    changes
}


impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let map = self.data.get_data();
        let mut occupation = vec![vec![false;map[0].len()];map.len()];
        loop{
            let val = step_map_two(map,& mut occupation);
//            println!("{} changes", val);
            if val == 0 {break;}
        }
        self.solution = Some(occupation.iter().map(|x| x.iter().filter(|&&y| y).count() as i64).sum())
    }
}