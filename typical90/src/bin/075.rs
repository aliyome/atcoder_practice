use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let primes = prime_factors(n);
    println!("{}", (primes.len() as f64).log2().ceil() as usize);
}

fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut div = 2;

    while div * div <= n {
        while (n % div) == 0 {
            factors.push(div);
            n /= div;
        }
        div += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}
