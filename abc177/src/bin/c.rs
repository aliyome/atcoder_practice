use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut sum = 0;
    let mut ans = 0;

    // First, calculate the sum of all elements.
    for &x in &a {
        sum += x;
        sum %= MOD;
    }

    // Then, for each element, subtract it from the sum and multiply.
    for &x in &a {
        sum = if sum >= x { sum - x } else { sum + MOD - x };
        ans += sum * x;
        ans %= MOD;
    }

    println!("{}", ans);
}
