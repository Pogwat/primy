fn main() {
    
    // x/(log(x))*(1+3/(2log(x))) overestimate of primes whithin a range GREATER THAN 1 by Rosser and Schoenfeld
    let range:u64 = 1234536650; // <----- Must be greater than 1 for this formula to work
    let n_primes_overestimate: usize =
    {
        let x = range as f64;
        (x/x.ln() ) * (1.0+3.0/ (2.0*x.ln()) )
    } as usize;
    
    println!("Over-Estimate of N primes: {}",n_primes_overestimate);

    let mut primes:Vec<u64> = Vec::with_capacity(n_primes_overestimate);
    primes.push(2);

    'numbers: for number in ( primes.last().unwrap()+1..range).step_by(2) {

        for &prime in &primes {
            if number%prime==0 { continue 'numbers;} 
            if prime*prime >number {  break;}
        }
        primes.push(number);

    }
    println!("{:?}", primes)
}

//This algo takes about 35minutes on my machine for a range of 1234536650