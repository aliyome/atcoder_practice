use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    // 0から55555までの素数を列挙
    let mut is_prime = vec![true; 55556];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..is_prime.len() {
        if is_prime[i] {
            for j in 2.. {
                if i * j >= is_prime.len() {
                    break;
                }
                is_prime[i * j] = false;
            }
        }
    }

    // 5n+1 の素数を列挙する
    // 5n+1 の数を 5個足すと、必ず5で割り切れる
    // 25(xxx + 5) % 5 == 0
    let mut primes = vec![];
    for i in 0..55556 {
        if is_prime[i] && i % 5 == 1 {
            primes.push(i);
        }
    }

    for i in 0..n {
        print!("{} ", primes[i]);
    }
    println!();
}
