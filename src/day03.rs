use super::*;
use super::puzzles::Data;


#[derive(Clone)]
pub struct Input{
    data: Vec<Vec<char>>
}

impl puzzles::Data<Vec<char>> for Input{
    fn new(input:&str) -> Input{
        Input{ data : parser::one_char_vec_per_line(input)}
    }
    fn get_data(&self) -> &Vec<Vec<char>>{
        &self.data
    }
}

pub struct Part1<'a>{
    data:&'a Input,
    solution: Option<u64>,
}
pub struct Part2<'a>{
    data:&'a Input,
    solution: Option<(u64,u64,u64,u64,u64)>,
}

impl<'a> std::fmt::Display for Part1<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.solution{
            Some(x) => {
                write!(f, "{} trees", x)
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
            Some((a,b,c,d,e)) => {
                write!(f, "{}, {}, {}, {} or {} trees for a product of {}",a,b,c,d,e,a*b*c*d*e )
            }
            None => {
                write!(f, "no solution found")
            }
        }
    }
}

impl<'a> puzzles::Puzzle<'a, Input> for Part1<'a>{
    fn new(input:&'a Input)->Self{
        Part1{data:input,solution:None}
    }
    fn resolve(&mut self){
        self.solution = Some(count_trees(self.data.get_data(), 3, 1));
    }
}
impl<'a> puzzles::Puzzle<'a, Input> for Part2<'a>{
    fn new(input:&'a Input)->Self{
        Part2{data:input,solution:None}
    }
    fn resolve(&mut self){
        self.solution = Some((
            count_trees(self.data.get_data(), 1, 1),
            count_trees(self.data.get_data(), 3, 1),
            count_trees(self.data.get_data(), 5, 1),
            count_trees(self.data.get_data(), 7, 1),
            count_trees(self.data.get_data(), 1, 2)
            ));
    }
}
fn count_trees(path:&Vec<Vec<char>>, x_step: usize, y_step:usize)->u64{
    let mut count = 0; 
    let mut x = 0;
    let width = path[0].len();
    for l in path.iter().step_by(y_step){
        if l[x%width] == '#'{
            count = count+1;
        }                
        x = x + x_step;
    }
    count
}