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
