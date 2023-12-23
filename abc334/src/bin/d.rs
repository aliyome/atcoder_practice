use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [usize; n],
        queries: [usize; q],
    }

    r.sort();

    // 累積和を計算
    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i + 1] = cum_sum[i] + r[i];
    }

    // 各クエリに対して二分探索で答えを探す
    for &x in queries.iter() {
        let (mut left, mut right) = (0, n + 1);
        while right - left > 1 {
            let mid = (left + right) / 2;
            if cum_sum[mid] <= x {
                left = mid;
            } else {
                right = mid;
            }
        }
        // トナカイの数がちょうどXか、それに最も近い小さい数を見つける
        while left > 0 && cum_sum[left] > x {
            left -= 1;
        }
        println!("{}", left);
    }
}
