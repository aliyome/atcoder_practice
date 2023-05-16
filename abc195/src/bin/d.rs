use proconio::input;

fn main() {
    input! {
        (n, m, q): (usize, usize, usize),
        wv: [(usize, usize); n],
        x: [usize; m],
        q: [(usize, usize); q],
    };

    // v->w の順でソート O(n log n)
    let mut wv = wv;
    wv.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    // w でソート O(m log m)
    let mut x = x.iter().enumerate().collect::<Vec<_>>();
    x.sort_by(|a, b| a.1.cmp(&b.1));

    // O(q * m * n) -> O(10^5)
    for (l, r) in q {
        let mut ans = 0;
        let mut used = vec![false; n];
        // 小さい箱から順に入れる
        for i in 0..m {
            // l から r までの箱は使用禁止
            let xi = x[i].0;
            if l - 1 <= xi && xi < r {
                continue;
            }

            // 価値の高い荷物から順に入れる
            for j in 0..n {
                if used[j] {
                    continue;
                }

                if wv[j].0 <= *x[i].1 {
                    ans += wv[j].1;
                    used[j] = true;
                    break;
                }
            }
        }

        println!("{}", ans);
    }
}
