use crate::calculator::evaluate;
use std::env;

mod calculator;

fn main() {
    let input = env::args().collect::<Vec<_>>().join(" ");
    let result = evaluate(&input);
    println!("{} = {}", input, result);
}
