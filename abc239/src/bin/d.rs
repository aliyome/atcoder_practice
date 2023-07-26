use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let is_prime = get_primes(200);

    for x in a..=b {
        let mut ok = true;
        for y in c..=d {
            if is_prime[x + y] {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
}

// エラトステネスの篩
fn get_primes(n: usize) -> Vec<bool> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;
    let mut i = 2;
    while i * i <= n {
        if primes[i] {
            let mut j = 2;
            while i * j <= n {
                primes[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    primes
}
