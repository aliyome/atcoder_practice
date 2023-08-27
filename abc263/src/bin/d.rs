use proconio::input;

fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        mut a: [isize; n]
    };

    // 1-indexed
    a.insert(0, 0);

    // aの累積和
    let mut acc = vec![0; n + 1];
    for i in 1..=n {
        acc[i] = acc[i - 1] + a[i];
    }

    // 全探索
    let mut ans = std::isize::MAX;
    // 左側の区間の長さ
    for il in 0..=n {
        // 左側の区間の総和
        let sum_l = l * il as isize;

        // 右側の区間の開始位置
        for ir in il + 1..=n {
            // 中間区間の総和
            let sum_m = acc[ir] - acc[il];
            // 右側区間の総和
            let sum_r = r * (n - ir) as isize;
            // 全体の総和
            let sum = sum_l + sum_m + sum_r;

            // 最小値を更新
            ans = ans.min(sum);
        }
    }

    println!("{}", ans);
}
