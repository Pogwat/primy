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
    let mut seive:FullSeive<4096> = FullSeive::new(123459);
    // println!("{:?}",seive.segmented_seive.next_local_multiples_iter(0,7).unwrap().into_iter().collect::<Vec<_>>());
    //seive.segmented_seive.remove_all_local_multiples_using_iter([3].iter());
    //seive.filter_local_for_primes();
    seive.filter_range();
    //println!("{:?}",seive.segmented_seive.segmented_seive);
    //println!("{:?}",seive.segmented_seive.find_some(seive.segmented_seive.current_idx));
    //println!("{:?}",seive.segmented_seive.next_local_multiples_idx(2,7).unwrap());
    //seive.segmented_seive.mut_next_local_multiples_iter(0,7).unwrap().for_each(|multiple| println!("{:?}",multiple) );
    //seive.filter_bump_seive();
    println!("{:?}",seive.primes.primes);
    
}