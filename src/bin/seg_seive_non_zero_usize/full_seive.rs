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
        self.segmented_seive.remove_multiples_in_iter(self.primes.iter_of_primes_to_check(self.segmented_seive.range));
        while let Some(prime_idx) = self.segmented_seive.find_some(self.segmented_seive.last_primes_idx.unwrap_or(0)) {
            self.segmented_seive.last_primes_idx = Some(prime_idx);
            if let Some(prime) = std::mem::take(&mut self.segmented_seive.segmented_seive[self.segmented_seive.last_primes_idx.unwrap()]) {
                let unwraped_prime = prime.get();
                self.primes.push(unwraped_prime);
                if unwraped_prime<=Primes::max_factor_to_check(self.segmented_seive.range) {
                self.segmented_seive.remove_multiples(unwraped_prime); } //Most primes are above sqrt range so we should on run this if the the prime is less than sqrt of range
            }
        }
    }

    pub fn filter_range(&mut self) {
        self.filter_local_for_primes();
        while self.segmented_seive.seg_end()<=self.segmented_seive.range {
            self.segmented_seive.bump_seive();
            self.filter_local_for_primes();
        }
    }
}

//not compiled: input: 1 billion, 389MB,  69.11s user 1.70s system 97% cpu 1:12.60 total

/* compiled: 
input: 1 billion, 389mb ram used, ./seg_seive_non_zero_usize 1000000000  7.91s user 1.62s system 83% cpu 11.454 total
input: 10 billion 3.4gb ram used, ./seg_seive_non_zero_usize 10000000000  121.81s user 15.09s system 87% cpu 2:36.98 total 
input: 40 billion, 11.3gb ram used, ./seg_seive_non_zero_usize 40000000000  932.48s user 73.86s system 92% cpu 18:06.41 total
*/