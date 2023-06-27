# Readme

[競プロ典型 90 問 - AtCoder](https://atcoder.jp/contests/typical90)を解く

## 用語

- 俯角: 水平面から下にある物を見る視線と水平面が成す角
- 仰角: 物を見上げた時の視線と水平面が成す角

## 計算量の雑な見積もり

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

![計算量の比較](./imgs/order.png)

## セグメント木

指定区間の最小値を得る、みたいなクエリで使える

- 構築に O(N)
- 区間に対するクエリに O(logN)

遅延評価セグメント木

- 区間加算: O(logN)
- 最小値計算: O(logN)

## 二分探索

O(logN)

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

### 平衡二分探索木

```rust
let mut set = BTreeSet::new();
set.insert(1); // O(logN)
set.remove(&1); // O(logN)
// 4以上を探したときの最初の要素を返す
set.range(4..).next();
// 7未満を探したときの最後の要素を返す
set.range(..7).last();
```

## 三分探索

たかだか一つしか極値のない関数における極値を探索するアルゴリズム。
つまり、二次関数のような下に凸な関数の最小値を求めることができる。
極値が複数ある場合は使えない。

範囲を 3 分することで、最小値がある範囲を狭めていく。

```cpp
  double left = -5;
  double right = 5;
  while(right-left>0.00000001) {
    double mid_l = left*2.0/3.0+right/3.0;
    double mid_r = left/3.0+right*2.0/3.0;
    if(func(mid_l)<func(mid_r)){
            right = mid_r;
        }else {
            left = mid_l;
        }
    }
```

## 組み合わせ

nC2 = n(n-1)/2

## Union-Find

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

## ヒープ

完全 2 分木

- 構築に O(N)
- 要素の追加・削除・更新に最悪でも O(logN)
- 最大値の取得に O(1)
- 要素数の取得に O(1)
- 親ノードは子ノードよりも常に大きい(小さい)という性質を持つ

## 最短経路

ダイクストラ法 O(ElogV)

- 重み付き有向グラフの単一始点最短経路問題を解くアルゴリズム
- 単純な DP を使うと O(V^2) で計算
- ヒープを使うと O(ElogV) で計算
- TODO: 負の重みを持つエッジがある場合は適用できない？？？

BFS O(E)

- コストが 1 の場合のみ使える

01-BFS O(E)

- ※後述 コストが 0 か 1 の場合のみ使える

Floyd-Warshall 法 O(V^3)

- 全頂点間の最短経路を求めるアルゴリズム
- dis[i][j] には i から j への最短経路のコストが入る。初期値は既知のコスト。存在しない経路は inf。

```rust
for k in 0..n {
    for i in 0..n {
        for j in 0..n {
            dis[i][j] = std::cmp::min(dis[i][j], dis[i][k] + dis[k][j]);
        }
    }
}
```

## DP の tips

- 値が bool の DP は bit DP で高速化できる(典型 90 問 012 の解説動画参照)
  - i 行目と i+1 行目の状態を OR すると i+1 行目の状態が得られる
    - 64 倍の高速化
- DP で辿った順序は、DP を埋めたあとに逆順に辿ると得られる
- 最長増加部分列(LIS: Longest Increasing Subsequence)は以下の 2 つを計算すると求められる
  - `dp[i]` i 番目まで見た時の LIS
  - `L[x]` 長さ x となる LIS の最後の要素の最小値

## 強連結成分分解(SCC)

Strongly Connected Components

- 有向グラフで互いの頂点に到達可能な時、強連結成分という。
- 深さ優先探索を 2 回する

1. DFS で帰りがけ順に番号を記録する
2. 辺の向きをすべて反転させる
3. 1.を逆順にする
4. 2.のグラフを 3.の順番に DFS し、到達した頂点を 1 つの強連結成分とする

## いもす法

- 重なり合う区間の数を累積和を使って数える方法
- 2 次元や 3 次元への拡張が容易
  - 例えば 2 次元なら O(NHW) かかるところを O(N+HW) で済ませられる

1. 始点にプラスの、終点にマイナスの重みをつける
2. 各軸の累積和を取る（これによりマイナスの重みをつけた部分は打ち消される）

![2dacc](./imgs/2d-acc.png)

## Nim のセオリーと Grundy 数

- 山が 2 つの時は、それぞれの山の石の数を同じ状態で手番が回ってきたら負け。つまり最初から山の数が同じ場合は先手が負け。
- 山が N の場合は、全山の石の数を XOR した値が 0 なら先手負け、0 以外なら先手勝ち。

参考: [組み合わせゲーム理論の基礎と Grundy 数での勝敗判定アルゴリズム | アルゴリズムロジック](https://algo-logic.info/combinatorial-games/#)

Nim のセオリー

- 山ごとに Grundy 数を求める
- それら Grundy 数すべてを XOR する
- 結果が 0 なら負け、0 以外なら勝ち

Grundy 数

- ある状態から遷移可能な状態の Grundy 数の集合の最小の非負整数
- 例えば、ある状態から遷移可能な状態の Grundy 数が {0, 1, 3} なら、その状態の Grundy 数は 2

## 重複組合せ

N-1 箇所の隙間から K-1 個の隙間を選んで仕切りを入れる場合の数は {N-1}_C_{K-1}

> o o o | o | o o o o | o o o
> N = 11, K = 4 の場合の一例 (10C3 通りある)

[AtCoder ABC 132 D - Blue and Red Balls (緑色, 400 点) - けんちょんの競プロ精進記録](https://drken1215.hatenablog.com/entry/2019/06/30/183400)

## オイラーツアー

DFS で木構造をたどる

## LCA

Lowest Common Ancestor

- 2 頂点の共通の祖先のうち、最も深いもの
- オイラーツアーを使うと O(N) で求められる
- 2 頂点(A, B)の距離を求められる
  - 根から A までの距離 + 根から B までの距離 - 2 x 根から LCA までの距離

## マンハッタン距離

|x1 - x2| + |y1 - y2|

以下のように 45 度回転させると

(u, v) = (x + y, x - y)

マンハッタン距離は以下のように表せる（チェビシェフ距離）

max(|u1 - u2|, |v1 - v2|)

証明

|x1 - x2| + |y1 - y2| = max(x1 - x2, x2 - x1) + max(y1 - y2, y2 - y1) とも表すことができるので、この右辺を式変形するとチェビシェフ距離になる。

## スライド最大値

Sliding Window Maximum

- 配列の連続する部分列の最大値を求める
- Deque を使うと O(N) で求められる
- 常に Deque の先頭が最大値になるようにする
- ウィンドウが右に移動する時に以下を行うことで Deque の先頭が常に最大になり、更新が O(1) でできる
  - 左端の値を削除
  - 右端の値を追加する時に、Deque の末尾から追加しようとしている値より小さな値を削除する

## 9 の倍数の性質

(N mod 9) = (N の各桁の和 mod 9)

## mod の性質

- (a + b) mod p = ((a mod p) + (b mod p)) mod p
- 足し算、引き算、掛け算はどのタイミングで mod を取っても答えは変わらない
- 割り算は mod を取る前に逆元を求めてから行う

## 01-BFS

- 辺の重みが 0 か 1 のグラフの最短経路を求める
- 通常の BFS ではなく、Deque を使う
- 重みが 0 の辺は先頭に、重みが 1 の辺は末尾に追加する
- 重みが 0 の辺は先に探索されるので、最短経路が求まる

## ベル数

n 個の要素を k 個のグループに分ける場合の数

- 例: 4 個の要素の場合、 {4}, {3, 1}, {2, 2}, {2, 1, 1}, {1, 1, 1, 1} の 5 通り
- ベル数の三角形を作ることで求められる

```text
   1
  1 2
 2 3 5
5 7 10 15
```

## パスカルの三角形

上から n 段目、左から k 番目の値は {n - 1}_C_{k - 1} である

```text
    1
   1 1
  1 2 1
 1 3 3 1
1 4 6 4 1
```

## bit 判断

以下は等価

```cpp
if ((bit & (1 << i)) != 0) {
  // i 番目の bit が立っている
}
if (((bit >> i) & 1) == 1) {
  // i 番目の bit が立っている
}
```

## 約数の個数

n の約数の個数は、n を素因数分解した時の指数の値に 1 を足したもの同士を掛け合わせたものになる
例: 24 = 2^3 x 3^1 なので、約数の個数は (3 + 1) x (1 + 1) = 8

```rust
// エラトステネスの篩を利用して n までの各整数の約数の個数を計算
fn divisor_counts(n: usize) -> Vec<usize> {
    let mut count = vec![0usize; n + 1];
    for i in 1..=n {
        let mut j = i;
        while j <= n {
            count[j] += 1;
            j += i;
        }
    }
    count
}
```

## 和の公式

- 1 から N までの総和は (N / 2) x (N + 1) で求められる
  - たとえば、1 から 50 の場合は 25 x 51 = 1275
  - これは、1 + 50 = 51, 2 + 49 = 51, 3 + 48 = 51, ... となるので、25 個の 51 で構成されるため
- a^0 + a^1 + a^2 + ... = 1/(1-a) | 0 < a < 1
  - 例えば、a = 0.5 の時、 1 + 0.5 + 0.25 + ... = 2

## 半分全列挙

全列挙すると TLE になる場合に、二分探索を使える状態にしてから全列挙するとうまく行くことがある。
例えば、A,B,C,D からそれぞれ 1 つずつ選んでその合計がある条件を満たすかを全列挙する場合、A,B と C,D に分けて考えることができる。O(N^3) -> O(N^2 log N)になる。

## 整数 X が 2 で何回割り切れるか

X を 2 進数表記して 1 のくらいから 0 が何回連続しているか
例: 32(10) = 0010 0000(2) なので、5 回
例: 33(10) = 0010 0001(2) なので、0 回

```rust
assert!(32usize.trailing_zeros() == 5);
```

## XOR の性質

- a ^ a = 0
- a ^ 0 = a
- a ^ b = b ^ a
- a ^ b ^ c = a ^ (b ^ c) = (a ^ b) ^ c
- a ^ b = c ならば a ^ c = b かつ b ^ c = a
- a + b = (a ^ b) + ((a & b) << 1)

## 数学

### 素数の求めかた(エラトステネスの篩)

[2,N]を列挙し、[2,√N]の倍数を除外する
計算量は O(NloglogN)

### 最大公約数の求めから(ユークリッドの互助法)

- 大きい方の数を「小さい方の数で割った余り」で置き換える
- 片方の数が 0 になったら操作終了、もう片方の数が答え
- 計算量は O(log(a + b))

### 最小公倍数

a と b の最小公倍数は ab / gcd(a, b)

### 繰り返し二乗法

以下の性質を使って a^n を高速に求める
a^1 x a^1 = a^2
a^2 x a^2 = a^4
a^4 x a^4 = a^8
︙

### 包除原理

x を a, b いずれかで割り切れるものの個数

=> x/a + x/b - x/ab

## Rust

```rust
// 配列の連続した重複要素の除去
a.sort();
a.dedup();
```

```rust
// BTreeSet
set.iter().next(); // 最小値
set.iter().last(); // 最大値
```

デカい数の a^b % MOD を計算可能な ModInt
仕組みは a^b は鉄則本 5.3 を参照

```rust
use std::ops;

const MOD: usize = 1000000007;

#[derive(Copy, Clone)]
pub struct ModInt {
    value: usize,
}

impl ModInt {
    pub fn new(value: usize) -> ModInt {
        ModInt { value: value % MOD }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn inverse(&self) -> ModInt {
        // (a, b) -> (x, y) s.t. a * x + b * y = gcd(a, b)
        fn extended_gcd(a: isize, b: isize) -> (isize, isize) {
            if (a, b) == (1, 0) {
                (1, 0)
            } else {
                let (x, y) = extended_gcd(b, a % b);
                (y, x - (a / b) * y)
            }
        }

        let (x, _y) = extended_gcd(self.value() as isize, MOD as isize);
        ModInt::new((MOD as isize + x) as usize)
    }

    // a^n
    pub fn pow(&self, mut n: usize) -> ModInt {
        let mut res = ModInt::new(1);
        let mut x = *self;
        while n > 0 {
            if n % 2 == 1 {
                res *= x;
            }
            x *= x;
            n /= 2;
        }

        res
    }
}

impl ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        ModInt::new(self.value + other.value)
    }
}

impl ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        ModInt::new(MOD + self.value - other.value)
    }
}

impl ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: Self) -> Self {
        ModInt::new(self.value * other.value)
    }
}

impl ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}

impl ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

// n!
pub fn factorial(n: usize) -> ModInt {
    (1..=n).fold(ModInt::new(1), |x, y| x * ModInt::new(y))
}

// nPr (https://ja.wikipedia.org/wiki/%E9%A0%86%E5%88%97#%E9%A0%86%E5%88%97%E3%81%AE%E6%95%B0%E3%81%88%E4%B8%8A%E3%81%92)
pub fn permutation(n: usize, r: usize) -> ModInt {
    if n < r {
        ModInt::new(0)
    } else {
        (n - r + 1..=n).fold(ModInt::new(1), |x, y| x * ModInt::new(y))
    }
}

// nCr (https://ja.wikipedia.org/wiki/%E7%B5%84%E5%90%88%E3%81%9B_(%E6%95%B0%E5%AD%A6))
pub fn combination(n: usize, r: usize) -> ModInt {
    if n < r {
        ModInt::new(0)
    } else {
        permutation(n, r) / factorial(r)
    }
}

// nHr (https://ja.wikipedia.org/wiki/%E9%87%8D%E8%A4%87%E7%B5%84%E5%90%88%E3%81%9B#%E9%87%8D%E8%A4%87%E7%B5%84%E5%90%88%E3%81%9B%E3%81%AE%E7%B7%8F%E6%95%B0)
pub fn homogeneous(n: usize, r: usize) -> ModInt {
    combination(n + r - 1, r)
}
```
