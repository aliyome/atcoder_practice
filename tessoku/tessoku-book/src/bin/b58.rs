use proconio::input;
use superslice::Ext;

fn main() {
    input! {
      n: usize, // <= 10^5
      l: isize,
      r: isize,
      mut x: [isize; n]
    }

    // 1-indexed
    x.insert(0, 0);

    // 貰うDP + セグ木
    let mut dp = vec![10_000_000_000usize; n + 1];
    let mut tree = SegmentTree::new(n + 1, |a, b| a.min(b));
    dp[1] = 0;
    tree.update(1, 0);

    // O(N)
    for i in 2..=n {
        let xl = x[i] - r;
        let xr = x[i] - l;
        // O(logN)
        let il = x.lower_bound(&xl);
        let ir = x.upper_bound(&xr);
        // 単純にl..rを見ていくとO(N)かかるので
        // セグ木でO(logN)にする
        // for j in il..ir {
        //     dp[i] = dp[i].min(dp[j] + 1);
        // }
        let min = tree.query(il, ir);
        dp[i] = dp[i].min(min + 1);
        tree.update(i, dp[i]);
    }

    println!("{}", dp[n]);
}

// 二分木を配列で表現するときは 1-indexed で表現したほうが都合が良い
// 深さと桁数が一致するようになる。 [セグメント木 [いかたこのたこつぼ]](https://ikatakos.com/pot/programming_algorithm/data_structure/segment_tree)
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
            return 10_000_000_000;
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

    fn _get(&self, pos: usize) -> usize {
        self.node[pos + self.size - 1]
    }

    fn _debug_print(&self) {
        for i in 0..self.size / 2 {
            print!("{} ", self.node[i + self.size]);
        }
        println!();
    }
}
