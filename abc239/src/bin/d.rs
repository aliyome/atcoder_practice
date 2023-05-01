use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    // 2-200までの素数を列挙
    let mut primes = vec![true; 200 + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=200 {
        if primes[i] {
            let mut j = 2 * i;
            while j <= 200 {
                primes[j] = false;
                j += i;
            }
        }
    }

    for takahashi in a..=b {
        let mut is_aoki_win = false;
        for aoki in c..=d {
            if primes[takahashi + aoki] {
                is_aoki_win = true;
                break;
            }
        }
        if is_aoki_win {
            continue;
        } else {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
}
