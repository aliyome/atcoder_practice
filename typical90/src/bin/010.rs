use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    // // 一番愚直な方法 O(N^2) -> TLE
    // for (l, r) in &lr {
    //     let mut sum = (0, 0);
    //     for (i, (c, p)) in cp.iter().enumerate() {
    //         if i + 1 < *l || *r < i + 1 {
    //             continue;
    //         }
    //         if *c == 1 {
    //             sum.0 += *p;
    //         } else {
    //             sum.1 += *p;
    //         }
    //     }
    // println!("{} {}", sum.0, sum.1);
    // }

    // 累積和を使う O(N + Q)
    let mut sum = vec![vec![0; n + 1]; 2 + 1];
    // 全生徒の累積和を計算
    for i in 0..n {
        sum[1][i + 1] = sum[1][i];
        sum[2][i + 1] = sum[2][i];
        sum[cp[i].0][i + 1] += cp[i].1;
    }
    // println!("{:?}", sum);
    // 指定区間を計算
    // println!("{:?}", lr);
    for (l, r) in &lr {
        let c1 = sum[1][*r] - sum[1][*l - 1];
        let c2 = sum[2][*r] - sum[2][*l - 1];
        println!("{} {}", c1, c2);
    }

    // セグメントツリーを使う
}
