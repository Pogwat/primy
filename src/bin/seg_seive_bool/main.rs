use std::num::NonZeroUsize;
mod primes;
mod seg;
mod full_seive;
use full_seive::FullSeive;
use std::env;
fn main() {
    let args:Vec<String> = env::args().collect();
    let mut range:usize = 2000000000; 
    if args.len()>1 { range = args[1].parse().unwrap_or(1000000);}
    let mut seive:FullSeive<65536> = FullSeive::new(range);
    seive.filter_range();
    println!("{:?}", seive.primes.primes);
}