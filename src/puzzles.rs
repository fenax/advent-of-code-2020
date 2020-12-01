use std::fmt::*;

pub trait Puzzle<T>: Display{
    fn new(input:&Vec<T>) -> Self;
    fn resolve(&mut self);
}
