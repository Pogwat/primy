pub struct Primes {
    pub primes:Vec<usize>
}

impl Primes {
    pub fn overestimate_num_of_primes(range:usize) -> usize {
        let x = range as f64;
        (x/(x.ln()-1.5)).ceil() as usize
    }

    pub fn new(range:usize) -> Self { 
        Self {
            primes:Vec::with_capacity(Self::overestimate_num_of_primes(range))
        }
    }

    pub fn max_factor_to_check(range:usize) -> usize {range.isqrt()}
    
    pub fn iter_of_primes_to_check(&self, range_max: usize) -> impl Iterator<Item = &usize> + '_ {
        let max_factor = Self::max_factor_to_check(range_max);
        self.primes.iter().take_while(move |&prime| *prime <= max_factor)     
    }
}

use std::num::NonZeroUsize;

//I Is type that holds Indexes in SEG_SIZE, Please ensure sizeof(I).=SEG_SIZE
pub struct MISPrimes<I,const SEG_SIZE:usize, const STEP:usize, const START_NUM:usize> {
    pub num_of_loops:usize,
    pub indexes: Vec<I>
}

impl  <I,const SEG_SIZE:usize, const STEP:usize,const START_NUM:usize>MISPrimes<I,SEG_SIZE,STEP,START_NUM> {

    //Montgomery-Vaughan theorem
    //Primes In seg<=2*SEG_SIZE/ ln(SEG_SIZE)

    pub fn max_primes_in_seg() -> usize {(2*SEG_SIZE)/(SEG_SIZE as f64).ln() as usize}
    
    pub fn new() ->Self {
        assert!(std::mem::size_of::<I>()>=SEG_SIZE);
        Self {
            num_of_loops:0,
            indexes: Vec::with_capacity(Self::max_primes_in_seg())
        }
    }

    pub fn mis_index_to_global_index(&self, mis_index:usize) -> usize {
        mis_index+SEG_SIZE*self.num_of_loops
    }

    pub fn get_prime(&self,mis_index:usize) -> usize {
        self.mis_index_to_global_index(mis_index)*STEP+START_NUM
    }
}