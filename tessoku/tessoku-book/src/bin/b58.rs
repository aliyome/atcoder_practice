use proconio::input;

fn main() {
    input! {
      n: usize,
      l: usize,
      r: usize,
      mut x: [isize; n]
    }

    // 1-indexed
    x.insert(0, 0);

    let mut seg = SegmentTree::new(n + 1, |a, b| a.min(b));
    seg.update(1, 0);

    // dp[i] := i番目の足場にたどり着いた時のジャンプ回数の最小値
    let mut dp = vec![0; n + 1];
    dp[1] = 0;
    for i in 2..n {
        let l = x.lower_bound(x[i] - r as isize);
        let r = x.lower_bound(x[i] - l as isize);
        dp[i] = std::usize::MAX;
        for j in l..r {
            dp[i] = dp[i].min(dp[j] + 1);
        }
    }

    println!("{}", dp[n]);
}
struct SegmentTree {
    size: usize,
    node: Vec<usize>,
    monoid: fn(usize, usize) -> usize,
}

impl SegmentTree {
    fn new(n: usize, monoid: fn(usize, usize) -> usize) -> Self {
        // ノードの数は 2 の指数倍に正規化する
        let mut size = 1;
        while size < n {
            size *= 2;
        }

        Self {
            size,
            // ルートが 1 の完全二分木なので、ノード数は 2n - 1
            // node[0] は使わない
            node: vec![0; size * 2],
            monoid,
        }
    }

    // pos 番目の値を x に更新する
    fn update(&mut self, pos: usize, x: usize) {
        // 葉のノードの位置
        let mut pos = pos + self.size - 1;
        self.node[pos] = x;
        // 親に上って更新していく
        while pos > 0 {
            pos /= 2;
            self.node[pos] = (self.monoid)(self.node[pos * 2], self.node[pos * 2 + 1]);
        }
    }

    // [l, r) の区間に対するクエリを実行する
    fn query(&self, l: usize, r: usize) -> usize {
        self._query(l, r, 1, self.size + 1, 1)
    }

    // [l, r) の区間に対するクエリを実行する
    // u は現在見ているノードの位置
    // [a, b) は u の担当区間
    // u が [l, r) に完全に含まれるまで分解していく
    fn _query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> usize {
        // ありえない区間なら 0 を返す
        // MEMO: 0 以外を返したい場合は実装を修正する。例えば、区間の最小値を求める場合は usize::MAX を返す。
        if r <= a || b <= l {
            return std::usize::MAX;
        }
        // u の担当区間がクエリ区間に完全に含まれるなら、u の値を返す
        if l <= a && b <= r {
            return self.node[u];
        }
        // そうでないなら、左右の子に対して再帰的にクエリを実行する
        let m = (a + b) / 2;
        let vl = self._query(l, r, a, m, u * 2);
        let vr = self._query(l, r, m, b, u * 2 + 1);
        // 左右の子の値をマージして返す
        (self.monoid)(vl, vr)
    }

    fn get(&self, pos: usize) -> usize {
        self.node[pos + self.size - 1]
    }
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
