use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    // O(N) 累積和
    let mut sum = vec![0; n + 1];
    for i in 1..n {
        if i % 2 == 0 {
            sum[i] = sum[i - 1] + a[i] - a[i - 1];
        } else {
            sum[i] = sum[i - 1];
        }
    }

    // println!("{:?}", sum);

    // O(N) すべてのクエリに対して
    for (l, r) in lr {
        // 最も近い時間を二分探索で求める
        let mut li = 0;
        let mut ri = 0;
        let mut ladd = 0;
        let mut radd = 0;
        match a.binary_search(&l) {
            Ok(i) => {
                li = i;
            }
            Err(i) => {
                li = i;
                if i % 2 == 0 && i > 0 {
                    ladd = a[i] - l;
                }
            }
        };

        match a.binary_search(&r) {
            Ok(i) => {
                ri = i;
            }
            Err(i) => {
                ri = i - 1;
                if i % 2 == 0 && i > 0 {
                    radd = r - a[i - 1];
                }
            }
        };

        let total = sum[ri] - sum[li] + ladd + radd;
        println!("{}", total);
        // println!("l:{} r:{} li:{} ri:{}", l, r, li, ri);
    }
}
