use super::*;
use super::puzzles::Data;
use std::vec::Vec;


#[derive(Clone)]
pub struct Input{
    data: Vec<Vec<char>>
}

impl puzzles::Data<Vec<Vec<char>>> for Input{
    // [z][y][x]
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
const directions:[(isize,isize,isize);26] = [(-1,-1,-1),(-1,-1,0),(-1,-1,1),
                                             (-1, 0,-1),(-1, 0,0),(-1, 0,1),
                                             (-1, 1,-1),(-1, 1,0),(-1, 1,1),
                                             ( 0,-1,-1),( 0,-1,0),( 0,-1,1),
                                             ( 0, 0,-1),          ( 0, 0,1),
                                             ( 0, 1,-1),( 0, 1,0),( 0, 1,1),
                                             ( 1,-1,-1),( 1,-1,0),( 1,-1,1),
                                             ( 1, 0,-1),( 1, 0,0),( 1, 0,1),
                                             ( 1, 1,-1),( 1, 1,0),( 1, 1,1),];
const directions4:[(isize,isize,isize,isize);80] = [
                                             (-1,-1,-1,-1),(-1,-1,-1,0),(-1,-1,-1,1),
                                             (-1,-1, 0,-1),(-1,-1, 0,0),(-1,-1, 0,1),
                                             (-1,-1, 1,-1),(-1,-1, 1,0),(-1,-1, 1,1),
                                             (-1, 0,-1,-1),(-1, 0,-1,0),(-1, 0,-1,1),
                                             (-1, 0, 0,-1),(-1, 0, 0,0),(-1, 0, 0,1),
                                             (-1, 0, 1,-1),(-1, 0, 1,0),(-1, 0, 1,1),
                                             (-1, 1,-1,-1),(-1, 1,-1,0),(-1, 1,-1,1),
                                             (-1, 1, 0,-1),(-1, 1, 0,0),(-1, 1, 0,1),
                                             (-1, 1, 1,-1),(-1, 1, 1,0),(-1, 1, 1,1),
                                             ( 0,-1,-1,-1),( 0,-1,-1,0),( 0,-1,-1,1),
                                             ( 0,-1, 0,-1),( 0,-1, 0,0),( 0,-1, 0,1),
                                             ( 0,-1, 1,-1),( 0,-1, 1,0),( 0,-1, 1,1),
                                             ( 0, 0,-1,-1),( 0, 0,-1,0),( 0, 0,-1,1),
                                             ( 0, 0, 0,-1),             ( 0, 0, 0,1),
                                             ( 0, 0, 1,-1),( 0, 0, 1,0),( 0, 0, 1,1),
                                             ( 0, 1,-1,-1),( 0, 1,-1,0),( 0, 1,-1,1),
                                             ( 0, 1, 0,-1),( 0, 1, 0,0),( 0, 1, 0,1),
                                             ( 0, 1, 1,-1),( 0, 1, 1,0),( 0, 1, 1,1),
                                             (1,-1,-1,-1),(1,-1,-1,0),(1,-1,-1,1),
                                             (1,-1, 0,-1),(1,-1, 0,0),(1,-1, 0,1),
                                             (1,-1, 1,-1),(1,-1, 1,0),(1,-1, 1,1),
                                             (1, 0,-1,-1),(1, 0,-1,0),(1, 0,-1,1),
                                             (1, 0, 0,-1),(1, 0, 0,0),(1, 0, 0,1),
                                             (1, 0, 1,-1),(1, 0, 1,0),(1, 0, 1,1),
                                             (1, 1,-1,-1),(1, 1,-1,0),(1, 1,-1,1),
                                             (1, 1, 0,-1),(1, 1, 0,0),(1, 1, 0,1),
                                             (1, 1, 1,-1),(1, 1, 1,0),(1, 1, 1,1),];




fn step_map(map:& mut Vec<Vec<Vec<char>>>)->i64{
    let old_map = map.clone();
    let mut changes = 0;
    let count_surrounding = |x: isize, y: isize, z:isize|{
        let depth = old_map.len()       as isize;
        let width = old_map[0][0].len() as isize;
        let height = old_map[0].len()   as isize;
        let read_map = |x:isize,y:isize,z:isize|->bool{
            if x >= 0 && x < width && y >=0 && y < height && z>=0&& z<depth{
                old_map[z as usize][y as usize][x as usize] == '#'
            }else{
                false
            }
        };
        let v : Vec<bool> = directions.iter().map(|(z_of,y_of,x_of)| read_map(x+x_of,y+y_of,z+z_of)).collect();
        let c = v.iter().filter(|&&x| x).count() as i64;
//        if c != 0 {println!("{:2}  {:2} {:2} {:2} {:?}",c,x,y,z,v);}
        c
    };
    for z in 0..map.len(){
        for y in 0..map[0].len(){
            for x in 0..map[0][0].len(){
                let count = count_surrounding(x as isize, y as isize, z as isize);
                if map[z][y][x] == '#'{//occupé
                    if count == 2 || count == 3{

                    }else{
                        changes += 1;
                        map[z][y][x] = '.';
                    }
                }else{//libre
                    if count == 3{
                        changes += 1;
                        map[z][y][x] = '#';
                    }
                }
            }
        }
    }
    changes
}
fn step_map_4d(map:& mut Vec<Vec<Vec<Vec<char>>>>)->i64{
    let old_map = map.clone();
    let mut changes = 0;
    let count_surrounding = |x: isize, y: isize, z:isize, w:isize|{
        let wow   = old_map.len()          as isize;
        let depth = old_map[0].len()       as isize;
        let width = old_map[0][0][0].len() as isize;
        let height = old_map[0][0].len()   as isize;
        let read_map = |x:isize,y:isize,z:isize,w:isize|->bool{
            if x >= 0 && x < width && y >=0 && y < height && z>=0&& z<depth && w>=0&& w<wow{
                old_map[w as usize][z as usize][y as usize][x as usize] == '#'
            }else{
                false
            }
        };
        let v : Vec<bool> = directions4.iter().map(|(w_of,z_of,y_of,x_of)| read_map(x+x_of,y+y_of,z+z_of,w+w_of)).collect();
        let c = v.iter().filter(|&&x| x).count() as i64;
//        if c != 0 {println!("{:2}  {:2} {:2} {:2} {:?}",c,x,y,z,v);}
        c
    };
    for w in 0..map.len(){
        for z in 0..map[0].len(){
            for y in 0..map[0][0].len(){
                for x in 0..map[0][0][0].len(){
                    let count = count_surrounding(x as isize, y as isize, z as isize, w as isize);
                    if map[w][z][y][x] == '#'{//occupé
                        if count == 2 || count == 3{

                        }else{
                            changes += 1;
                            map[w][z][y][x] = '.';
                        }
                    }else{//libre
                        if count == 3{
                            changes += 1;
                            map[w][z][y][x] = '#';
                        }
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
        let input = self.data.get_data();
        let mut map = vec![vec![vec!['.';input[0].len()+14];input.len()+14];15];
        for i in (0..input.len()){
            for j in (0..input[0].len()){
                map[7][i+7][j+7] = input[i][j];
            }
        }
        for _ in 0..6{
            let val = step_map(& mut map);
            println!("{} changes", val);
        }
        self.solution = Some(map.iter().map(|z| z.iter().map(|y| y.iter().filter(|x| **x == '#').count() as i64).sum::<i64>() as i64).sum())
    }
}


impl<'a,'b> puzzles::Puzzle<'a, Input> for Part2<'b>
where 'a:'b
{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        let input = self.data.get_data();
        let mut map = vec![vec![vec![vec!['.';input[0].len()+14];input.len()+14];15];15];
        for i in (0..input.len()){
            for j in (0..input[0].len()){
                map[7][7][i+7][j+7] = input[i][j];
            }
        }
        for _ in 0..6{
            let val = step_map_4d(& mut map);
            println!("{} changes", val);
        }
        self.solution = Some(map.iter().map(|w|
                               w.iter().map(|z| 
                                z.iter().map(|y| 
                                 y.iter().filter(|x| **x == '#').count() as i64)
                                .sum::<i64>() as i64)
                                .sum::<i64>() as i64)
                            .sum())
    }
}