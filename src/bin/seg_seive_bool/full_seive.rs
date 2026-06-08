use crate::seg::SegSeive;
use crate::primes::Primes;

pub struct FullSeive<const SEG_SIZE:usize> {
    pub seive: SegSeive<SEG_SIZE>,
    pub primes:Primes
}

impl <const SEG_SIZE:usize> FullSeive<SEG_SIZE> {
    
    pub fn new(range:usize) -> Self {
        let mut result = Self {
            seive: SegSeive::<SEG_SIZE>::new(range),
            primes: Primes::new(range)
        };
        result.primes.push(3);
        result
    }




} 