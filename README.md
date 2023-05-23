# Readme

[競プロ典型 90 問 - AtCoder](https://atcoder.jp/contests/typical90)を解く

## Memo

### 用語

- 俯角: 水平面から下にある物を見る視線と水平面が成す角
- 仰角: 物を見上げた時の視線と水平面が成す角

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

遅延評価セグメント木

- 区間加算: O(logN)
- 最小値計算: O(logN)

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

### 強連結成分分解(SCC)

Strongly Connected Components

- 有向グラフで互いの頂点に到達可能な時、強連結成分という。
- 深さ優先探索を 2 回する

1. DFS で帰りがけ順に番号を記録する
2. 辺の向きをすべて反転させる
3. 1.を逆順にする
4. 2.のグラフを 3.の順番に DFS し、到達した頂点を 1 つの強連結成分とする

### いもす法

- 重なり合う区間の数を累積和を使って数える方法
- 2 次元や 3 次元への拡張が容易
  - 例えば 2 次元なら O(NHW) かかるところを O(N+HW) で済ませられる

1. 始点にプラスの、終点にマイナスの重みをつける
2. 各軸の累積和を取る（これによりマイナスの重みをつけた部分は打ち消される）

### Nim のセオリーと Grundy 数

参考: [組み合わせゲーム理論の基礎と Grundy 数での勝敗判定アルゴリズム | アルゴリズムロジック](https://algo-logic.info/combinatorial-games/#)

Nim のセオリー

- 山ごとに Grundy 数を求める
- それら Grundy 数すべてを XOR する
- 結果が 0 なら負け、0 以外なら勝ち

Grundy 数

- ある状態から遷移可能な状態の Grundy 数の集合の最小の非負整数
- 例えば、ある状態から遷移可能な状態の Grundy 数が {0, 1, 3} なら、その状態の Grundy 数は 2

### 重複組合せ

N-1 箇所の隙間から K-1 個の隙間を選んで仕切りを入れる場合の数は {N-1}_C_{K-1}

> o o o | o | o o o o | o o o
> N = 11, K = 4 の場合の一例 (10C3 通りある)

[AtCoder ABC 132 D - Blue and Red Balls (緑色, 400 点) - けんちょんの競プロ精進記録](https://drken1215.hatenablog.com/entry/2019/06/30/183400)
