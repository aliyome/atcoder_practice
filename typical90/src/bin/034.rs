use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut count = HashMap::new();
    let mut ans = 0;
    let mut l = 0;
    for r in 0..n {
        *count.entry(a[r]).or_insert(0) += 1;

        // k 種類以内の場合
        if count.len() <= k {
            ans = ans.max(r - l + 1);
            continue;
        }

        // k 種類を超えた場合
        // 一番古いものを削除して、文字種が変わるまで l を進める
        while count.len() > k {
            count.entry(a[l]).and_modify(|e| *e -= 1);
            if count[&a[l]] == 0 {
                count.remove(&a[l]);
            }
            l += 1;
        }
        ans = ans.max(r - l + 1);
    }

    println!("{}", ans);
}
