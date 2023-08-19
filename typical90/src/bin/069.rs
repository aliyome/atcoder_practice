use proconio::input;
const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    // k(k-1)(k-2)^{n-2}
    if n == 1 {
        println!("{}", k % MOD);
    } else if n == 2 {
        if k < n {
            println!("0");
        } else {
            println!("{}", (k * (k - 1)) % MOD);
        }
    } else if k < 3 {
        println!("0");
        return;
    } else {
        let x = (k * (k - 1)) % MOD;
        let y = binpower(k - 2, n - 2) % MOD;
        let ans = (x * y) % MOD;
        println!("{}", ans);
    }
}

fn binpower(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut ans = 1usize;
    while b != 0 {
        if b % 2 == 1 {
            ans = (ans * a) % MOD;
        }
        a = (a * a) % MOD;
        b /= 2;
    }
    return ans;
}
