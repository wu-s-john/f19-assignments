mod part1;
mod part2;
extern crate rand;
use rand::prelude::*;

fn main() { part1::password_checker(String::from("Password1")); }