use std::env;
mod primes;
mod array_collect;
mod segmented_seive;
mod full_seive;
use crate::full_seive::FullSeive;

fn main() {
    let args:Vec<String> = env::args().collect();
    let mut range:usize = 2000000000; 
    if args.len()>1 { range = args[1].parse().unwrap_or(1000000);}
    let mut seive:FullSeive<4096> = FullSeive::new(range);
    seive.filter_range();
    println!("{:?}, {}",seive.primes.primes, seive.segmented_seive.seg_end());
    //println!("{:?}",seive.segmented_seive.segmented_seive);
}