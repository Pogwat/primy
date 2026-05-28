use delegate::delegate;
use std::ops::Index;
use std::ops::IndexMut;
use std::env;
use std::fmt;
fn main() {
    let args:Vec<String> = env::args().collect();
    
    // x/(log(x))*(1+3/(2log(x))) overestimate of primes whithin a range GREATER THAN 1 by Rosser and Schoenfeld
    let mut range:usize = 2000000000; // <----- Must be greater than 1 for this formula to work
    if args.len()>1 { range = args[1].parse().unwrap_or(1000000);}

    type SieveArray<const SIZE: usize> = [Option<usize>; SIZE / std::mem::size_of::<usize>()];

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


    fn array_from_iter<T, const SIZE: usize>(mut iter: impl Iterator<Item = T>) -> [T;SIZE] {
        let array:[T;SIZE] = std::array::from_fn(|_| iter.next().unwrap() );
        array
    }











    struct Seive<const SIZE: usize > {
            seg_seive: [Option<usize>; SIZE / std::mem::size_of::<usize>()],
            primes: Vec<usize>,
            current_idx:usize,
            segment_start:usize,
            step:usize,
            range:usize,
    }

    impl <const Size:usize>fmt::Debug for Seive<Size> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        f.debug_list()
            .entries(self.primes.iter())
            .finish()?;
        write!(f, ", current_checked_prime_idx: {}, current_idx: {} }}", 
        self.primes.len()-1, self.current_idx
        )
    }}

    // 1. Delegate the read-only Index trait
    impl <const Size:usize>Index<usize> for Seive <Size>{
        type Output = Option<usize>;

        delegate! {
            to self.seg_seive {
                fn index(&self, index: usize) -> &Self::Output;
            }
        }
    }

    // 2. Delegate the mutable IndexMut trait
    impl <const Size: usize> IndexMut<usize> for Seive<Size> {
        delegate! {
            to self.seg_seive {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output;
            }
        }
    }

    impl<const SIZE: usize> Seive<SIZE> {
        fn new(range:usize) -> Self {
            const segment_start:usize=3;
            const step:usize=2;
            let seg_end_num:usize= {Seive::NUMBER_OF_ELEMENTS_PER_SEG}*step+segment_start;
           
            Self {
                seg_seive: array_from_iter::<Option<usize>, {Seive::NUMBER_OF_ELEMENTS_PER_SEG}>((segment_start..seg_end_num).step_by(step).map(|num| Some(num as usize))),
                primes: vec![3],
                current_idx:0,
                segment_start,
                step,
                range,
            }
        }
        
        const SIZE:usize = SIZE;
        const NUMBER_OF_ELEMENTS_PER_SEG: usize = SIZE / std::mem::size_of::<usize>();

        fn guess_dex(&self,index:usize)-> usize {self.segment_start+index*self.step}
        fn seg_end(&self) -> usize{ self.seg_seive.last().and_then(|&opt| opt).unwrap_or(self.guess_dex(self.seg_seive.len()-1))}
        fn fast_calc_at_index(&self, idx:usize) -> usize {self.step*idx+self.segment_start}
        
        fn new_seg_array(&mut self) {
            self.seg_seive = array_from_iter((self.segment_start..self.seg_end()).step_by(self.step).map(|num| Some(num as usize)))
        }

        fn checked_remove_multiple(&mut self, multiple: usize, start_idx: usize) {       
            if start_idx < self.len() { //Dosent go out of bounds even if start_idx+multiple>self.len()
                (start_idx+multiple..self.len()).step_by(multiple).for_each(|idx| self[idx] = None);
            }
        }

        fn bump_seive(&mut self) {
            //self.segement_size/
        }

        delegate! {
            to self.seg_seive {
                pub fn len(&self) -> usize;
            }
        }

        // fn remove_all_multiple_of_current_prime(&mut self) {
        //     self.checked_remove_multiple(self[self.current_checked_prime_idx].unwrap(),self.current_checked_prime_idx)
        // }

    }