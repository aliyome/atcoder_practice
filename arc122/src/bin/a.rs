use proconio::input;

const MOD: isize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n]
    };

    a.insert(0, 0);

    let mut ans = vec![];

    dfs(0, 0, '+', n, &a, &mut ans);

    let mut sum = 0;
    for &ans in &ans {
        sum += ans;
        sum %= MOD;
    }

    println!("{}", sum);
}

fn dfs(i: usize, sum: isize, op: char, n: usize, a: &[isize], ans: &mut Vec<isize>) {
    if i == n - 1 {
        if op == '+' {
            ans.push((sum + a[i + 1]) % MOD);
        } else {
            ans.push((sum - a[i + 1]) % MOD);
        }
        return;
    }

    let mut sum = sum;
    if op == '+' {
        sum += a[i + 1];
        sum %= MOD;
        dfs(i + 1, sum, '+', n, a, ans);
        dfs(i + 1, sum, '-', n, a, ans);
    } else {
        sum -= a[i + 1];
        sum %= MOD;
        dfs(i + 1, sum, '+', n, a, ans);
    }
}
