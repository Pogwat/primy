use delegate::delegate;
use std::ops::Index;
use std::ops::IndexMut;
use std::env;
use std::fmt;
use std::ops::Range;

use ArrayCollectExt as AIT;
fn main() {
    let args:Vec<String> = env::args().collect();
    
    // x/(log(x))*(1+3/(2log(x))) overestimate of primes whithin a range GREATER THAN 1 by Rosser and Schoenfeld
    let mut range:usize = 2000000000; // <----- Must be greater than 1 for this formula to work
    if args.len()>1 { range = args[1].parse().unwrap_or(1000000);}

    // let leftover = 1..3;
    let mut seive: Seive<4096> = Seive::new(range);

    let test_val:usize =36;
    let multiple = 2;
    let next_multiple = match (test_val.next_multiple_of(multiple)) {
        val if val == test_val => {test_val+multiple}
        _ => {test_val.next_multiple_of(multiple)}
    }; 
    println!("{}",next_multiple);
    println!("{}, {}",seive.guess_dex(0), seive.seg_seive[0].unwrap());
    println!("{}",seive.seg_seive.len());
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
            step:usize,
            range:usize,
            num_of_loops:usize
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
                step:STEP,
                range,
                num_of_loops:0
            }
        }
        
        const FIRST_START_NUM:usize=3;
        const SIZE:usize = SIZE;

        fn overestimate_num_of_primes(range:usize) -> usize {
            let x = range as f64;
            (x/(x.ln()-1.5)).ceil() as usize
        }

        fn guess_dex(&self,index:usize)-> usize {(self.num_of_loops*SIZE+index)*self.step+Self::FIRST_START_NUM}
 
        fn seg_start(&self) -> usize {self.guess_dex(0)}
        fn seg_end(&self) -> usize {self.guess_dex(self.seg_seive.len()-1)}
        fn is_value_within_seg(&self, value:usize) -> bool{ value <= self.seg_end() && value >= self.seg_start() }
        
        fn seg_start_globdex(&self) -> usize {self.num_of_loops*SIZE}
        fn seg_end_globdex(&self) -> usize {self.seg_start_globdex()+SIZE-1}
        fn is_global_idx_within_seg(&self, global_idx:usize) -> bool {
            global_idx>= self.seg_start_globdex() && global_idx<= self.seg_end_globdex()
        }

        fn global_idx_to_local_idx(&self, global_idx:usize) -> Option<usize>{
            return if self.is_global_idx_within_seg(global_idx) {
                Some(global_idx-self.seg_start_globdex())
            } else {None}
        }

        fn local_value_to_local_idx(&self,local_value:usize) -> usize{ (local_value-self.seg_start())/self.step }
        fn local_idx_to_global_idx(&self, local_idx:usize) -> usize {self.seg_start_globdex()+local_idx}
        
        fn global_value_to_global_idx(&self, global_value:usize) -> usize {(global_value-Self::FIRST_START_NUM)/self.step}
        fn global_value_to_local_idx(&self, global_value:usize) -> Option<usize> {
            return if self.is_value_within_seg(global_value) {
                Some(self.local_value_to_local_idx(global_value))
            } else {None}
        }

        fn find_next_multiple_local_idx(&self, multiple:usize, start:Option<usize>) -> Option<usize>{
            let local_start_idx = start.unwrap_or(0);
            let start_value = self.guess_dex(local_start_idx);
            let next_multiple = match (start_value.next_multiple_of(multiple)) {
                match_ if match_ == start_value => {start_value+multiple}
                _ => {start_value.next_multiple_of(multiple)}
            }; 
            self.global_value_to_local_idx(next_multiple)
            
        }

        // fn multiples_mut_iter(&mut self,start:Option<usize> ) -> impl Iterator<Item = &mut usize> {

        // }

        fn bump_sieve(&mut self) -> &[usize] {
            let new_start_num = self.guess_dex(self.seg_seive.len()-1)+self.step;
            let old_seive_primes_range = self.append_to_primes_somes();
            self.seg_seive = (new_start_num..).step_by(self.step).map(|num| Some(num as usize)).collect_array().unwrap();
            self.num_of_loops+=1;
            self.current_idx=0;
            &self.primes[old_seive_primes_range]
        }

        fn append_to_primes_somes(&mut self) -> Range<usize> {
            let mut seg_somes = self.seg_seive.into_iter().flatten().collect();
            let old_primes_len = self.primes.len();
            self.primes.append(&mut seg_somes);
            old_primes_len..self.primes.len()
        }

        fn find_some(&mut self) -> Option<usize> {
            self.seg_seive.iter().skip(self.current_idx).position(|num| num.is_some()).map(|relative_idx| self.current_idx + relative_idx)
        }

        // while let Some(prime) =  self.find_some() {

        //     self.current_idx+=1;
        // }


        delegate! {
            to self.seg_seive {
                pub fn len(&self) -> usize;
            }
        }

    }