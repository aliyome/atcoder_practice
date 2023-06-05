use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k, p): (usize, usize, usize),
        a: [usize; n],
    }

    // 全通り
    // 最大 40C20 = 137846528820 通り -> 10^11 -> TLE
    let all = (0..n).combinations(k).collect_vec();
    println!("{}", all.len());
    // それぞれの通りの合計値
    let mut ans = 0;
    for list in all.iter() {
        let mut sum = 0;
        for &i in list {
            sum += a[i];
        }
        if sum <= p {
            ans += 1;
        }
    }
    println!("{}", ans);
}
