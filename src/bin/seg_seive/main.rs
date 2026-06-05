use std::env;
use std::ops::Range;

mod primes;
mod array_collect;
mod segmented_seive;
mod full_seive;

use crate::segmented_seive::SegmentedSeive;
use crate::primes::Primes;
use crate::full_seive::FullSeive;

fn main() {
    let args:Vec<String> = env::args().collect();
    
    // x/(log(x))*(1+3/(2log(x))) overestimate of primes whithin a range GREATER THAN 1 by Rosser and Schoenfeld
    let mut range:usize = 2000000000; // <----- Must be greater than 1 for this formula to work
    if args.len()>1 { range = args[1].parse().unwrap_or(1000000);}
    let mut seive:FullSeive<4096> = FullSeive::new(range);
    seive.filter_range();
    println!("{:?}",seive.primes.primes);
    
}