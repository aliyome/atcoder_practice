use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("0");
        return;
    }

    // 全事象 10^n
    let mut total = 1;
    for _ in 0..n {
        total *= 10;
        total %= MOD;
    }

    let mut x = 1;
    for _ in 0..n {
        x *= 8;
        x %= MOD;
    }

    let mut y = 1;
    for _ in 0..n {
        y *= 9;
        y %= MOD;
    }
    y *= 2;
    y %= MOD;

    let mut ans = total;
    ans = (MOD + ans - y) % MOD;
    ans = (MOD + ans + x) % MOD;

    println!("{}", ans);
}
