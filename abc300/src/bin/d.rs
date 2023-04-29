use proconio::input;

const MAX_N: usize = 1000000;

fn furui() -> Vec<usize> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; MAX_N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=MAX_N {
        if is_prime[i] {
            primes.push(i);
            let mut j = 2 * i;
            while j <= MAX_N {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

fn main() {
    input! {
        n: usize,
    };

    let primes = furui();
    let mut ans = 0;

    for i in 0..primes.len() {
        let a = primes[i];
        if a * a * a * a * a > n {
            break;
        }
        for j in (i + 1)..primes.len() {
            let b = primes[j];
            if a * a * b * b * b > n {
                break;
            }
            for k in (j + 1)..primes.len() {
                let c = primes[k];
                if a * a * b * c * c <= n {
                    ans += 1;
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", ans);

    // let mut count = 0;
    // for i in 0..primes.len() {
    //     let a = primes[i];
    //     if (a as u64) * (a as u64) * 2 > n {
    //         break;
    //     }
    //     for j in (i + 1)..primes.len() {
    //         let b = primes[j];
    //         if (a as u64) * (a as u64) * (b as u64) * (b as u64) > n {
    //             break;
    //         }
    //         for k in (j + 1)..primes.len() {
    //             let c = primes[k];
    //             if (a as u64) * (a as u64) * (b as u64) * (c as u64) * (c as u64) <= n {
    //                 count += 1;
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    // }

    // println!("{}", count);
}
