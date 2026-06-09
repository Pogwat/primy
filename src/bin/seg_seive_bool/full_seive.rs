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
        result.primes.primes.push(3);
        result
    }

    pub fn filter_primes(&mut self) {
        self.seive.remove_multiples_in_iter(self.primes.iter_of_primes_to_check(self.seive.range));
        while let Some(prime_idx) = self.seive.find_set(self.seive.last_prime_idx.unwrap_or(0)) {
            let prime_val = self.seive.guess_dex(prime_idx);
            self.primes.primes.push(prime_val);
            self.seive.seive[prime_idx] = false;
            if prime_val<=Primes::max_factor_to_check(self.seive.range) {
                self.seive.remove_multiple(prime_val);
            }
            self.seive.last_prime_idx = Some(prime_idx);
        }
    }

    pub fn filter_range(&mut self) {
        self.filter_primes();
        for _ in 0..self.seive.num_of_seive_bumps() {
            self.seive.bump_seive();
            self.filter_primes();
        }
        
    }
} 