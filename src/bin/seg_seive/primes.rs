pub struct Primes {
    primes: Vec<usize>
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
}