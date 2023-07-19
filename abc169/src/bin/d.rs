use std::collections::{BTreeMap, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    // n を素因数分解する
    // let mut primes = vec![];
    // for i in 2..=(n as f64).sqrt().ceil() as usize {
    //     if n % i == 0 {
    //         primes.push(i);
    //     }
    // }
    let primes = prime_factors(n);

    let mut candidates = BTreeMap::new();
    for p in primes {
        for i in 1.. {
            if p.overflowing_pow(i as u32).1 {
                break;
            }
            if p.pow(i) > 1_000_000_000_000 {
                break;
            }
            *candidates.entry(p.pow(i)).or_insert(0) += 1;
        }
    }

    let mut ans = 0;
    let mut curr = n;
    let mut used = HashMap::new();
    for (&k, &v) in candidates.iter() {
        if v - *used.get(&k).unwrap_or(&0) == 0 {
            continue;
        }

        if curr % k == 0 {
            ans += 1;
            curr /= k;
            *used.entry(k).or_insert(0) += 1;
        }
    }

    println!("{}", ans);
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
