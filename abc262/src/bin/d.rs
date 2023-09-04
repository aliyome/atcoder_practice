use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut ans = 0;
    dfs(0, n, 0, 0, &a, &mut ans);
    println!("{}", ans);
}

fn dfs(i: usize, n: usize, sum: usize, chosen: usize, a: &[usize], ans: &mut usize) {
    if i == n {
        return;
    }

    // 1. 選ばない
    dfs(i + 1, n, sum, chosen, a, ans);

    // 2. 選ぶ
    let sum = (sum + a[i]) % MOD;
    let chosen = chosen + 1;
    if sum % chosen == 0 {
        *ans += 1;
    }
    dfs(i + 1, n, sum, chosen, a, ans);
}
