use delegate::delegate;
use std::ops::Index;
use std::ops::IndexMut;
use std::env;
use std::fmt;

use ArrayCollectExt as AIT;
fn main() {
    let args:Vec<String> = env::args().collect();
    
    // x/(log(x))*(1+3/(2log(x))) overestimate of primes whithin a range GREATER THAN 1 by Rosser and Schoenfeld
    let mut range:usize = 2000000000; // <----- Must be greater than 1 for this formula to work
    if args.len()>1 { range = args[1].parse().unwrap_or(1000000);}

    // let leftover = 1..3;
    let mut seive: Seive<4096> = Seive::new(range);
 

    // 1. Correct the upper bound calculation for the loop
    // let limit = ((range as f64).sqrt() as usize / seive.spacing );
    // for number_to_check in (0..limit) {
    //     if let Some(prime) = seive[number_to_check] {
    //         seive.current_checked_prime_idx=number_to_check;
    //         seive.remove_all_multiple_of_current_prime();
    //     }
    // }
    // println!("{:?}", seive)
}

//12.5Gb, 2 billion, 14 minutes 40 seconds

pub trait ArrayCollectExt: Iterator + Sized {
    // The method takes a const generic size parameter `SIZE`
    fn collect_array<const SIZE: usize>(&mut self) -> Option<[Self::Item; SIZE]> {
        let mut error = false;
        
        let array = std::array::from_fn(|_| {
            match self.next() {
                Some(val) => val,
                None => {
                    error = true;
                    // Fallback to satisfy from_fn signature if iterator is short
                    unsafe { std::mem::zeroed() }
                }
            }
        });

        if error { None } else { Some(array) }
    }
}

// 2. Blanket implement it for ALL iterators automatically
impl<I: Iterator> ArrayCollectExt for I {}

    struct Seive<const SIZE: usize> {
            seg_seive: [Option<usize>;SIZE],
            primes: Vec<usize>,
            current_idx:usize,
            segment_start:usize,
            step:usize,
            range:usize
    }

    impl <const SIZE: usize>fmt::Debug for Seive<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        f.debug_list()
            .entries(self.primes.iter())
            .finish()?;
        write!(f, ", current_checked_prime_idx: {}, current_idx: {} }}", 
        self.primes.len()-1, self.current_idx
        )
    }}

    // 1. Delegate the read-only Index trait
    impl <const SIZE: usize> Index<usize> for Seive<SIZE> {
        type Output = Option<usize>;

        delegate! {
            to self.seg_seive {
                fn index(&self, index: usize) -> &Self::Output;
            }
        }
    }

    // 2. Delegate the mutable IndexMut trait
    impl <const SIZE: usize> IndexMut<usize> for Seive<SIZE> {
        delegate! {
            to self.seg_seive {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output;
            }
        }
    }

    impl<const SIZE: usize> Seive<SIZE> {
        fn new(range:usize) -> Self {
            const START_RANGE:usize=3;
            const STEP:usize=2;
            let seg_end_num:usize= SIZE*STEP+START_RANGE;
           
            Self {
                seg_seive: (START_RANGE..seg_end_num).step_by(STEP).map(|num| Some(num as usize)).collect_array().unwrap(),
                primes: Vec::with_capacity(Self::overestimate_num_of_primes(range)),
                current_idx:0,
                segment_start:START_RANGE,
                step:STEP,
                range,
            }
        }
        
        const START_RANGE:usize=3;
        const SIZE:usize = SIZE;

        fn overestimate_num_of_primes(range:usize) -> usize {
            let x = range as f64;
            (x/(x.ln()-1.5)).ceil() as usize
        }

        fn guess_dex(&self,index:usize)-> usize {self.segment_start+index*self.step}
        
        fn seg_end(&self) -> usize{ 
            self.seg_seive.last().and_then(|&opt| opt).unwrap_or(self.guess_dex(self.seg_seive.len()-1))
        }
        
        fn new_seg_array(&mut self) -> &[Option<usize>;SIZE] {
            self.seg_seive = (self.segment_start..self.seg_end()).step_by(self.step).map(|num| Some(num as usize)).collect_array().unwrap();
            &self.seg_seive
        }

        fn find_first_multiple(&self, multiple:usize, start:Option<usize>) -> Option<usize>{
            let multiple_fac_closest_to_start = self.segment_start.div_ceil(multiple);
            self.global_value_to_local_idx(multiple_fac_closest_to_start*multiple+start.unwrap_or(0))
        }

        fn find_upper_multiple(&self,multiple:usize, start_idx:usize) -> Option<usize> {
            let start_val = self.guess_dex(start_idx);
            let next_multiple_global_val = if start_val % multiple == 0 {
                start_val + multiple
            } else {
                start_val.next_multiple_of(multiple)
            };
            self.global_value_to_local_idx(next_multiple_global_val)
        }

        fn loop_num(&self) -> usize {
            (self.segment_start-Self::START_RANGE)/self.step/SIZE
        }

        fn global_idx(&self, value:usize) -> usize {
            (value-Self::START_RANGE)/self.step
        }

        fn global_value_to_local_idx(&self,value:usize) -> Option<usize> {
            if value>= self.segment_start && value <=self.seg_end() {
            Some(self.unchecked_value_to_local_idx(value))
            } else {None}
        }

        fn unchecked_value_to_local_idx(&self,value:usize) -> usize {
            (value-self.segment_start)/self.step
        }

        // fn first_multiple_in_seive(&self, multiple:usize) -> Option<usize> {
        //     if multiple<= seg.end() {
        //         if let Some(valid_idx) =global_value_to_local_idx(multiple) {
        //             return Some(valid_idx)
        //         } else {
        //             let invalid_idx = unchecked_value_to_local_idx(multiple);

        //         }
                
                 
        //     }
        // }
 
        fn remove_multiples(&mut self, multiple: usize, start:usize) -> &[Option<usize>;SIZE] {       
            self.seg_seive.iter_mut().skip(start).step_by(multiple).for_each(|num| *num = None);
            &self.seg_seive
        }

        fn bump_seive(&mut self) -> &[Option<usize>;SIZE]{
            let last_num = self.guess_dex(self.seg_seive.len()-1);
            self.segment_start = last_num+self.step;
            let new_seg:[Option<usize>; SIZE] = (self.segment_start..(self.segment_start+SIZE)).step_by(self.step).map(|num| Some(num as usize)).collect_array().unwrap();
            self.seg_seive = new_seg;
            &self.seg_seive
        }

        // fn sieve_segment(&mut self) {
        //     let segment_end = self.segment_start + SIZE;
        //     for &prime in self.primes {
        //         // 1. Dynamically find the first multiple >= segment_start
        //         let mut multiple = ((self.segment_start + prime - 1) / prime) * prime;

        //         // 2. Protect the prime itself from being crossed off
        //         if multiple == prime {
        //             multiple += prime;
        //         }

        //         // 3. Cross off all multiples within this segment
        //         while multiple < segment_end {
        //             let local_idx = (multiple - segment_start);
        //             sieve[local_idx] = None;
        //             multiple += prime; // Step to the next multiple
        //         }

        //         //step*n + FIRST_START_RANGE_NUM + segment_start = num
        //         //(num - segment_start - FIRST_START_RANGE_NUM)/step = N
        //         let global_idx =  self.global_idx(num);


        //     }
        // }
        
        fn local_multiple_iter(&self, multiple:usize, start_idx:Option<usize>) Option<impl Iterator<Item = usize> > {
            let start_idx = start_idx.unwrap_or(0);
            let first_multiple_val = self.seg_seive[start_idx].next_multiple_of(multiple);
            
        }


        fn append_to_primes_somes(&mut self) -> &[usize] {
            let mut seg_somes = self.seg_seive.into_iter().flatten().collect();
            let old_primes_len = self.primes.len();
            self.primes.append(&mut seg_somes);
            &self.primes[old_primes_len..]
        }

        fn find_some(&mut self) -> Option<usize> {
            self.seg_seive.iter().skip(self.current_idx).position(|num| num.is_some()).map(|relative_idx| self.current_idx + relative_idx)
        }

        // fn calculate(&mut self) -> Vec<usize> {
        //     'eliminate_loop: while let Some(some_idx) = self.find_some.is_some() {
        //         self.current_idx = some_idx;
        //         self.remove_multiples(self.seg_seive[self.current_idx]);
        //     }
        //     append_to_primes_somes.



             
        

        delegate! {
            to self.seg_seive {
                pub fn len(&self) -> usize;
            }
        }

    }