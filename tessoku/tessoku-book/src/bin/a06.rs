use proconio::input;

fn main() {
    input! {
      n: usize,
      q: usize,
      a: [isize; n],
      lr: [(usize, usize); q],
    }

    // 1日目からの累積和を計算
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] += acc[i] + a[i];
    }

    for (l, r) in lr {
        // l日目からr日目までの合計を計算
        let ans = acc[r] - acc[l - 1];
        println!("{}", ans);
    }
}
