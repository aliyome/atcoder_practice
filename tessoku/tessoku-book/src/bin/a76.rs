use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
      n: usize, // <= 1.5 x 10^5
      w: usize, // <= 10^9
      l: usize,
      r: usize,
      mut x: [usize; n] // x <= 10^9
    }

    // 西岸を0、東岸をwとする
    x.insert(0, 0);
    x.push(w);

    // // dp[i] := i番目の足場に来る方法の数
    // let mut dp = vec![0usize; n + 2];
    // dp[0] = 1;

    // // 素朴にDPを実装した場合
    // // O(N^2) -> TLE

    // // O(NlogN)になるように、二分探索を使う
    // // 配るDP
    // for i in 0..=n {
    //     let xl = x.lower_bound(x[i] + l);
    //     let xr = x.upper_bound(x[i] + r);
    //     // この部分のループは単純に足しているだけなので高速化できそう -> 累積和
    //     for j in xl..xr {
    //         dp[j] += dp[i];
    //         dp[j] %= MOD;
    //     }
    // }

    // 貰うDP, 累積和
    let l = l as isize;
    let r = r as isize;
    let x = x.into_iter().map(|x| x as isize).collect::<Vec<_>>();
    let mut dp = vec![0isize; n + 2];
    let mut sum = vec![0isize; n + 2];
    dp[0] = 1;
    sum[0] = 1;

    for i in 1..=n + 1 {
        // 範囲外の足場がない場合はスキップ
        let ir = x.lower_bound(x[i] - l) as isize;
        let il = x.lower_bound(x[i] - r) as isize;

        println!("{} {} {}", i, il, ir);

        let sumxl = if x[i] <= r { 0 } else { sum[il - 1] };

        dp[i] = (sum[ir] - sumxl) % MOD as isize;
        sum[i] = (sum[i - 1] + dp[i]) % MOD as isize;
    }

    println!("{:?}", dp);
    println!("{:?}", sum);

    println!("{}", dp[n + 1]);
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, key: T) -> usize;
    fn upper_bound(&self, key: T) -> usize;
}

impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    // key以上の値が初めて現れる位置
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

    // keyより大きい値が初めて現れる位置
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
