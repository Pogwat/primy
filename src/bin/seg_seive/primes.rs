use delegate::delegate;
pub struct Primes {
    pub primes: Vec<usize>
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

    pub fn max_factor_to_check(prime:usize) -> usize {prime.isqrt()}
    
    pub fn iter_of_primes_to_check(&self, range_max: usize) -> impl Iterator<Item = &usize> + '_ {
        let limit = Self::overestimate_num_of_primes(range_max);
        self.primes.iter().take_while(move |&prime| *prime < limit)     
    }

    delegate!{
        to self.primes {
            pub fn len(&self) -> usize;
            pub fn push(&mut self, value: usize);
            pub fn append(&mut self, other: &mut Vec<usize>);
        }
    }
}