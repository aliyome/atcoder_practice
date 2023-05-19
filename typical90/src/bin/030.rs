use proconio::input;

// 解答見た
fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut cnt = vec![0; n + 1];
    for i in 2..=n {
        if cnt[i] >= 1 {
            continue;
        }
        for j in (i..=n).step_by(i) {
            cnt[j] += 1;
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        if cnt[i] >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
