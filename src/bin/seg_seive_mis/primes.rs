use std::num::NonZeroUsize;

//I Is type that holds Indexes in SEG_SIZE, Please ensure sizeof(I).=SEG_SIZE
pub struct MISPrimes<I,const SEG_SIZE:usize, const STEP:usize, const START_NUM:usize> {
    pub num_of_loops:Vec<Range<I>>, //Range of segemnts local indexes
    pub indexes: Vec<I> //All local indexes i.e. 1 3 5 ,7,1 3 5 ,7 ...
}



impl  <I,const SEG_SIZE:usize, const STEP:usize,const START_NUM:usize>MISPrimes<I,SEG_SIZE,STEP,START_NUM> {
    pub fn overestimate_num_of_primes(range:usize) -> usize {
        let x = range as f64;
        (x/(x.ln()-1.5)).ceil() as usize
    }

    pub fn new(range:usize) ->Self {
        assert!(2.pow(std::mem::size_of::<I>()*8)>=SEG_SIZE);
        Self {
            num_of_loops:Vec::with_capacity(range/STEP/SEG_SIZE),
            indexes: Vec::with_capacity(Self::overestimate_num_of_primes(range))
        }
    }

    pub fn mis_index_to_global_index(&self, mis_index:usize) -> usize {
        mis_index+SEG_SIZE*self.num_of_loops
    }

    pub fn get_prime(&self,mis_index:usize) -> usize {
        self.mis_index_to_global_index(mis_index)*STEP+START_NUM
    }

    pub fn max_factor_to_check(range:usize) -> usize {range.isqrt()}
    
    // pub fn iter_of_primes_to_check(&self, range_max: usize) -> impl Iterator<Item = &usize> + '_ {
    //     let max_factor = Self::max_factor_to_check(range_max);
    //     self.primes.iter().take_while(move |&prime| *prime <= max_factor)     
    // }
}

