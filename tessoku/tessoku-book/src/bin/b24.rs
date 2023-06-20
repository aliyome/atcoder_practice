use proconio::input;

fn main() {
    input! {
      n: usize,
      mut xy: [(usize, usize); n]
    }

    // x で昇順ソート
    xy.sort_by(|a, b| a.0.cmp(&b.0));
    xy.insert(0, (0, 0));

    // dp[i] := i 番目まで見た時のLIS
    let mut dp = vec![0; n + 1];
    // l[len] := 長さ len のLISの最後の要素の最小値
    let mut l = vec![10usize.pow(10); n + 1];

    // 初期値
    dp[0] = 0;
    l[0] = 0;

    // DP
    for i in 1..=n {
        let (_, y) = xy[i];
        let len = l.lower_bound(y);
        dp[i] = len;
        l[len] = y;
    }

    // 出力
    let mut max = 0;
    for i in 1..=n {
        max = max.max(dp[i]);
    }
    println!("{}", max);
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, key: T) -> usize;
    fn upper_bound(&self, key: T) -> usize;
}

impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    fn lower_bound(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key <= self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    fn upper_bound(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key < self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}
