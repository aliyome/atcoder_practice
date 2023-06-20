use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    let is_primes = sieve(n);
    for i in 2..=n {
        if is_primes[i] {
            println!("{}", i);
        }
    }
}

fn sieve(n: usize) -> Vec<bool> {
    let mut is_primes = vec![true; n + 1];
    is_primes[0] = false;
    is_primes[1] = false;

    for i in 2..=(n as f64).sqrt() as usize {
        if !is_primes[i] {
            continue;
        }

        let mut j = i + i;
        while j <= n {
            is_primes[j] = false;
            j += i;
        }
    }

    is_primes
}
