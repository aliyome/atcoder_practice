use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let mut pow10 = vec![0usize; 20];
    pow10[0] = 1;
    for i in 1..20 {
        pow10[i] = pow10[i - 1] * 10;
    }

    // f(n) := n * (n + 1) / 2 % MOD
    let f = |n: usize| -> usize {
        if n % 2 == 0 {
            let v1 = (n / 2) % MOD;
            let v2 = (n + 1) % MOD;
            (v1 * v2) % MOD
        } else {
            let v1 = ((n + 1) / 2) % MOD;
            let v2 = n % MOD;
            (v1 * v2) % MOD
        }
    };

    //
    let mut ans = 0;
    for i in 1..=19 {
        let l = l.max(pow10[i - 1]);
        let r = r.min(pow10[i] - 1);
        if l > r {
            continue;
        }
        let v = (MOD + f(r) - f(l - 1)) % MOD;
        ans = (ans + v * i) % MOD;
    }

    println!("{}", ans);
}

// a^b
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
