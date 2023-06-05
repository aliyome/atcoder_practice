use proconio::input;

fn main() {
    input! {
      n: isize, // 1 <= n <= 3000
      k: isize // 3 <= k <= 9000
    }

    // 全探索 O(n^2)
    let mut ans = 0;
    for r in 1..=n {
        for b in 1..=n {
            let w = k - (r + b);
            if w < 1 || n < w {
                continue;
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}
