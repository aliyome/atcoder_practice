use proconio::input;

fn main() {
    input! {
      n: usize, // <= 10^5
      mut a: [usize; n] // a[i] <= 5*10^5
    }

    // 1-indexed
    a.insert(0, std::usize::MAX);

    // dp[i] := ai が最後の要素となる最長の増加部分列の長さ
    let mut dp = vec![0usize; n + 1];
    dp[0] = 0;
    // l[x] := 長さが x の増加部分列における最終要素の最小値
    // l[dp[i]] := 長さが dp[i] の増加部分列における最終要素の最小値 = ai
    let mut l = vec![std::usize::MAX; n + 1];
    l[0] = 0;
    let mut len = 0;

    // a[i] を順に見ていく
    for i in 1..=n {
        // a[i] より小さい最大の l[x] を探す。 lower_bound なので pos = x + 1
        let pos = l.lower_bound(a[i]);
        // dp[i] は x + 1
        dp[i] = pos;

        // l[x + 1] = a[i];
        l[dp[i]] = a[i];
        len = len.max(dp[i]);
    }

    println!("{}", len);
    // println!("{:?}", dp);
    // println!("{:?}", l);
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
