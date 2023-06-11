use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    };

    // 座標圧縮
    // map := 圧縮前の値 -> 圧縮後の値
    // 1 は圧縮しない
    let mut comp2orig = HashMap::new();
    let mut orig2comp = HashMap::new();
    let mut i = 2;
    *orig2comp.entry(1).or_insert(1) = 1;
    *comp2orig.entry(1).or_insert(1) = 1;
    for &(a, b) in &ab {
        if a != 1 && !orig2comp.contains_key(&a) {
            orig2comp.insert(a, i);
            comp2orig.insert(i, a);
            i += 1;
        }
        if b != 1 && !orig2comp.contains_key(&b) {
            orig2comp.insert(b, i);
            comp2orig.insert(i, b);
            i += 1;
        }
    }

    // 座標圧縮後の値の最大値を取得
    let n = *orig2comp.values().max().unwrap();
    // println!("{:?}", orig2comp);
    // println!("{}", n);

    // UnionFind でグループ分け
    let mut uf = UnionFind::new(n + 2);
    for &(a, b) in &ab {
        // println!("a: {}, b: {}", a, b);
        let a = *orig2comp.get(&a).unwrap();
        let b = *orig2comp.get(&b).unwrap();
        // println!("a: {}, b: {}", a, b);
        uf.unite(a, b);
    }

    // 1 と同じグループに属する値の最大値を取得
    let mut max = 1;
    for i in 1..=n {
        if uf.same(1, i) {
            max = max.max(*comp2orig.get(&i).unwrap());
        }
    }

    println!("{}", max);
}

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
