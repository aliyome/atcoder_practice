use proconio::input;

const MOD: isize = 998244353;

fn main() {
    input! {
        n: isize,
    };

    if n % MOD < 0 {
        println!("{}", (n % MOD) + MOD);
    } else {
        println!("{}", (n % MOD));
    }
}
