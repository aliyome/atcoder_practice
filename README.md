# Readme

[競プロ典型 90 問 - AtCoder](https://atcoder.jp/contests/typical90)を解く

## Memo

### 計算量の雑な見積もり

１秒間で処理できる計算ステップ回数は 109 ＝ 1,000,000,000 回程度

- O(log N): いっぱい
- O(N): 10^8
- O(N log N): 10^7
- O(N^2): 10^4
- O(N^3): 300
- O(2^N): 20
- O(N!): 10

指数時間: O(2^N) とか O(N!) とか
多項式時間: O(N^2) とか O(N^3) とか

2^10 -> 1024: だいたい 10^3

### セグメント木

指定区間の最小値を得る、みたいなクエリで使える

- 構築に O(N)
- 区間に対するクエリに O(logN)

### 二分探索

```rust
let v = vec![0, 1, 2, 3, 4, 5, 6];
let i = v.binary_search_by(|&x| {
    if x <= 3 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}).unwrap_err();
// i == 4 となる
```

### 組み合わせ

nC2 = n(n-1)/2

### Union-Find

- 構築に O(N)

```rust
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    // O(N)
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    // O(α(N)) ≒ O(1)
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
            p
        }
    }

    // O(α(N)) ≒ O(1)
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.parent[y] = x;
            self.size[x] += self.size[y];
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    // O(α(N)) ≒ O(1)
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
```

### ヒープ

完全 2 分木

- 構築に O(N)
- 要素の追加・削除・更新に最悪でも O(logN)
- 最大値の取得に O(1)
- 要素数の取得に O(1)
- 親ノードは子ノードよりも常に大きい(小さい)という性質を持つ

### 最短経路

ダイクストラ法

- 重み付き有向グラフの単一始点最短経路問題を解くアルゴリズム
- 単純な DP を使うと O(V^2) で計算
- ヒープを使うと O(ElogV) で計算
- TODO: 負の重みを持つエッジがある場合は適用できない？？？

### DP の tips

- 値が bool の DP は bit DP で高速化できる(典型 90 問 012 の解説動画参照)
  - i 行目と i+1 行目の状態を OR すると i+1 行目の状態が得られる
    - 64 倍の高速化
