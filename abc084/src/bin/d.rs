use proconio::input;

fn main() {
    input! {
        q: usize,
        lr: [(usize, usize); q]
    };

    let is_prime = calc_primes(100_000);

    let mut like2017 = vec![0; 100_001];
    for i in 2..=100_000 {
        like2017[i] = if is_prime[i] && is_prime[(i + 1) / 2] {
            1
        } else {
            0
        };
    }

    // 累積和
    let mut acc = vec![0; 100_001];
    for i in 2..=100_000 {
        acc[i] = acc[i - 1] + like2017[i];
    }

    for (l, r) in lr {
        println!("{}", acc[r] - acc[l - 1]);
    }
}

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
