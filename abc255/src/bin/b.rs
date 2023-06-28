use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; k],
        mut xy: [(isize, isize); n],
    };

    // 1-indexed
    a.insert(0, 0);
    xy.insert(0, (0, 0));

    // torch_dists[i] := i から最も近い松明までの距離
    let mut torch_dists = vec![std::f64::MAX; n + 1];

    // すべての松明について
    for i in 1..=k {
        // 松明の位置
        let (x1, y1) = xy[a[i]];
        // 松明の位置から各人の距離を計算
        for j in 1..=n {
            if a[i] == j {
                torch_dists[j] = 0.0f64;
                continue;
            }
            // 人の位置
            let (x2, y2) = xy[j];
            // 松明と人の距離
            let dist = (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
            // 人から最も近い松明までの距離を更新
            torch_dists[j] = torch_dists[j].min(dist);
        }
    }

    let mut max = std::f64::MIN;
    for i in 1..=n {
        max = max.max(torch_dists[i]);
    }

    println!("{}", max);
}
