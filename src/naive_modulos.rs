use std::env;
fn main() {
   
    let args: Vec<String> = env::args().collect();
    let mut prime_range = 1000;
    if args.len()>1 {if let Ok(range) =  args[1].parse::<u64>() {
        prime_range = range;
    } } 


    let _leftover = 1;
    let mut primes_exclude_1: Vec<u64> = vec![2];
    'numbers: for number in ((primes_exclude_1[primes_exclude_1.len()-1]+1)..prime_range).step_by(2) {
        
        for &prime in &primes_exclude_1 {
            if number%prime == 0 {
                continue 'numbers
            }
            if prime*prime>number {
                break;
            }
        }

        primes_exclude_1.push(number);

    }
    println!("{:?}", primes_exclude_1);
}