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
            result.primes.push(3);
            result

        }

        pub fn filter_local_for_primes(&mut self) {
            self.segmented_seive.remove_all_local_multiples_using_iter(self.primes.primes.iter());
            
            while let Some(prime_idx) = self.segmented_seive.find_some(self.segmented_seive.current_idx) {
                self.segmented_seive.current_idx = prime_idx;
                if let Some(val) = std::mem::take(&mut self.segmented_seive.segmented_seive[self.segmented_seive.current_idx]) {
                    self.primes.push(val)
                }
                self.segmented_seive.remove_all_local_multiples_using_iter(self.primes.iter_of_primes_to_check(self.segmented_seive.seg_end()))
            }
        }

        pub fn filter_range(&mut self) {
            self.filter_local_for_primes();
            while let Some(new_start) = self.segmented_seive.bump_seive() {
                self.filter_local_for_primes();
            }
            //println!("{:?}, {:?}", self.segmented_seive.bump_seive(), self.segmented_seive.segmented_seive[0])
        }
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