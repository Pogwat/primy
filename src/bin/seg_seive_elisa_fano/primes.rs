use std::num::NonZeroUsize;
use num_traits::{PrimInt, Unsigned};

//I Is type that holds Indexes in SEG_SIZE, Please ensure sizeof(I).=SEG_SIZE
pub struct MISPrimes<I:Sized,const SEG_SIZE:usize, const STEP:usize, const START_NUM:usize>
where
    I: PrimInt + Unsigned + std::ops::AddAssign, // This restricts I to unsigned integers only!
{
    pub seg_end_indexes:Vec<usize>,  //range of seg indexes
    pub indexes: Vec<I> //All local indexes i.e. 1 3 5 ,7,1 3 5 ,7 ...
}



impl  <I,const SEG_SIZE:usize, const STEP:usize,const START_NUM:usize>MISPrimes<I,SEG_SIZE,STEP,START_NUM> 
where
    I: PrimInt + Unsigned + std::ops::AddAssign, // This restricts I to unsigned integers only!
{
    pub fn overestimate_num_of_primes(range:usize) -> usize {
        let x = range as f64;
        (x/(x.ln()-1.5)).ceil() as usize
    }

    pub fn new(range:usize) ->Self {
        Self {
            num_of_primes_per_loop:Vec::with_capacity(range/STEP/SEG_SIZE),
            indexes: Vec::with_capacity(Self::overestimate_num_of_primes(range))
        }
    }

    pub fn global_index_to_value(mis_index:usize) -> usize {mis_index*STEP+START_NUM}

    pub fn mut_num_of_primes_in_loop(&mut self, loop_index:usize) -> &mut I {
        &mut self.num_of_primes_per_loop[loop_index]
    } 

    pub fn last_loop_index(&self) -> usize {self.num_of_primes_per_loop.len()}

    pub fn new_loop(&mut self, inital_value:I) {self.num_of_primes_per_loop.push(inital_value)}



    pub fn get_prime(&self,mis_index:usize) -> Option<usize> {
        if let Some(globdex) = self.mis_index_to_global_index(mis_index)
        {Some(Self::global_index_to_value(globdex))} else {None}
    }

    pub fn max_factor_to_check(range:usize) -> usize {range.isqrt()}

    pub fn itet() {}
    
    // pub fn iter_of_primes_to_check(&self, range_max: usize) -> impl Iterator<Item = &usize> + '_ {
    //     let max_factor = Self::max_factor_to_check(range_max);
    //     self.primes.iter().take_while(move |&prime| *prime <= max_factor)     
    // }
}

// Your custom iterator from the previous step
pub struct SieveIndexIterator<'a, I> {
    pub indexes: &'a [I],
    pub current_idx: usize,
    pub loops: usize,
}

impl<'a, I> Iterator for SieveIndexIterator<'a, I> {
    type Item = &'a I;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.indexes.len() {None} else {

            let val_ref = &self.array_ref[self.current_idx];
            self.current_idx += 1;
            Some(val_ref)
        }
    }
}

// --- THE IMPLICIT MAGIC ---
// Implement IntoIterator for a reference to your struct
impl<'a, I> IntoIterator for &'a MISPrimesFixed<I> {
    type Item = &'a I;
    type IntoIter = SieveIndexIterator<'a, I>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SieveIndexIterator {
            indexes: &self.indexes,
            current_idx: 0,
            loops: self.num_of_primes_per_loop,
        }
    }
}


