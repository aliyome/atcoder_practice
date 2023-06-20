use proconio::input;

fn main() {
    input! {
      q: usize,
      x: [usize; q]
    }

    let primes = calc_primes(3 * 10usize.pow(5));

    for &x in &x {
        // if is_prime(x) {
        if primes[x] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

// // TLE
// fn is_prime(n: usize) -> bool {
//     for i in 2..n {
//         if n % i == 0 {
//             return false;
//         }
//     }
//     true
// }

// // N まで調べる必要はない。√N まで調べれば十分。
// fn is_prime(n: usize) -> bool {
//     for i in 2..(n as f64).sqrt() as usize + 1 {
//         if n % i == 0 {
//             return false;
//         }
//     }
//     true
// }

// エラトステネスの篩
fn calc_primes(n: usize) -> Vec<bool> {
    let mut is_primes = vec![true; n + 1];
    is_primes[0] = false;
    is_primes[1] = false;

    for i in 2..=(n as f64).sqrt() as usize {
        if !is_primes[i] {
            continue;
        }

        let mut j = i * 2;
        while j <= n {
            is_primes[j] = false;
            j += i;
        }
    }

    is_primes
}
