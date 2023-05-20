use proconio::input;
use proconio::marker::Isize1;
use proconio::marker::Usize1;
use std::collections::HashMap;
use std::collections::HashSet;

struct UnionFind {
    parent: Vec<i32>,
    size: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: vec![-1; n],
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            x
        } else {
            self.parent[x] = self.root(self.parent[x] as usize) as i32;
            self.parent[x] as usize
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x != y {
            if self.size[y] < self.size[x] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent[x] = y as i32;
            self.size[y] += self.size[x];
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        sets: [(usize, [Usize1; sets.0]) for sets in [(Usize1)].iter().cloned().collect::<Vec<(Usize1,)>>(); n],
    }

    let mut uf = UnionFind::new(m);
    let mut group = vec![HashSet::new(); m];
    for (i, set) in sets.into_iter().enumerate() {
        let mut iter = set.into_iter();
        let first = iter.next().unwrap();
        for x in iter {
            uf.unite(first, x);
        }
    }

    for i in 0..m {
        let root = uf.root(i);
        group[root].insert(i);
    }

    let mut check = vec![false; m];
    let mut que = vec![];
    let mut dist = vec![std::usize::MAX; m];
    dist[0] = 0;
    que.push(0);
    while let Some(v) = que.pop() {
        if check[v] {
            continue;
        }
        check[v] = true;
        for &nv in &group[v] {
            if !check[nv] {
                que.push(nv);
                dist[nv] = dist[v] + 1;
            }
        }
    }

    let answer = if check[m - 1] { dist[m - 1] - 1 } else { -1 };
    println!("{}", answer);
}
