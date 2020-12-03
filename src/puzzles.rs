use std::fmt::*;

pub trait Data<T>{
    fn new(input:&str) ->Self;
    fn get_data(&self) -> &Vec<T>;
}

pub trait Puzzle<'r,T>: Display
{
    fn new(input:&'r T) -> Self;
    fn resolve(&mut self);
}
