use crate::segmented_seive::SegmentedSeive;
use crate::primes::Primes;

pub struct FullSeive<const SEG_SIZE: usize> {
        pub segmented_seive: SegmentedSeive<SEG_SIZE>,
        pub primes: Primes
    }

    impl<const SEG_SIZE: usize> FullSeive<SEG_SIZE> {
        pub fn new(range:usize) -> Self {
            let mut result = Self {
                segmented_seive: SegmentedSeive::<SEG_SIZE>::new(range),
                primes: Primes::new(range)
            };
            result.primes.push(2);
            result

        }

        pub fn remove_segs_primes(&mut self){
            let primes_to_check_iter = self.primes.iter_of_primes_to_check(self.segmented_seive.seg_end());
            self.segmented_seive.remove_all_multiples_in_iter(primes_to_check_iter);    
        }

        pub fn recusrisve_remove_prime_multiples(&mut self) {
            if let Some(new_start) = self.segmented_seive.find_some(self.segmented_seive.current_idx+1) {
                self.segmented_seive.current_idx=new_start;
                self.primes.push(self.segmented_seive.guess_dex(self.segmented_seive.current_idx));
                self.remove_segs_primes();
            } 
        }

        pub fn flatten_append(&mut self) {
            let mut flattened_seive = self.segmented_seive.segmented_seive.iter_mut().filter_map(|opt| opt.take()).collect();
            self.primes.primes.append(&mut flattened_seive);
        }

        pub fn filter_bump_seive(&mut self) {
            self.recusrisve_remove_prime_multiples();
            self.flatten_append();
        }

        // fn bump_sieve(&mut self) -> &[usize] {
        //     let seive_primes_index_range = self.drain_seive_to_primes_return_primes_index_range();
        //     self.segmented_seive = (self.seg_end()+self.step..=self.seg_end()+SIZE*self.step).step_by(self.step).map(|num| Some(num as usize)).collect_array().unwrap();
        //     self.num_of_loops+=1;
        //     self.current_idx=0;
        //     &self.primes[seive_primes_index_range]
        // }

        // fn drain_seive_to_primes_return_primes_index_range(&mut self) -> Range<usize> {
        //     let mut seg_somes = self.segmented_seive.into_iter().flatten().collect();
        //     let old_primes_len = self.primes.len();
        //     self.primes.append(&mut seg_somes);
        //     old_primes_len..self.primes.len()
        // }

        // fn get_all_primes_in_seg(&mut self) {
        //         // Loop by index to avoid borrowing self.primes
        //     for prime_idx in 0..self.primes.len() {
        //         let prime = self.primes[prime_idx]; // Copy the value out (no borrow held)
        //         if prime > Self::max_factor_to_check(prime) { break; }
        //         // Create a scoped block to control exactly when the iterator is dropped
        //          self.current_idx = {
        //             if let Some(mut prime_multiples_iter) = self.mut_multiple_iter(prime, self.current_idx) 
        //                 && let Some(peek) = prime_multiples_iter.peek() 
        //             {
        //                 let val = peek.unwrap();
        //                 prime_multiples_iter.into_iter().for_each(|multiple| *multiple = None);
        //                 val
        //             } else {self.current_idx}
        //         }; // prime_multiples_iter and its borrow on `self` are completely dropped HERE
        //     }
        // }
        


        // delegate! {
        //     to self.segmented_seive {
        //         pub fn len(&self) -> usize;
        //     }
        // }

    }

       // impl <const SIZE: usize>fmt::Debug for Seive<SIZE> {
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
    //     f.debug_list()
    //         .entries(self.primes.iter())
    //         .finish()?;
    //     write!(f, ", current_checked_prime_idx: {}, current_idx: {} }}", 
    //     self.primes.len()-1, self.current_idx
    //     )
    // }}

    // // 1. Delegate the read-only Index trait
    // impl <const SIZE: usize> Index<usize> for Seive<SIZE> {
    //     type Output = Option<usize>;

    //     delegate! {
    //         to self.segmented_seive {
    //             fn index(&self, index: usize) -> &Self::Output;
    //         }
    //     }
    // }

    // // 2. Delegate the mutable IndexMut trait
    // impl <const SIZE: usize> IndexMut<usize> for Seive<SIZE> {
    //     delegate! {
    //         to self.segmented_seive {
    //             fn index_mut(&mut self, index: usize) -> &mut Self::Output;
    //         }
    //     }
    // }