use std::fmt::*;

pub trait Data<T>{
    fn new(input:&str) ->Self;
    fn get_data(&self) -> &Vec<T>;
}

pub trait Puzzle<T>: Display
{
    fn new(input:&T) -> Self;
    fn resolve(&mut self);
}
