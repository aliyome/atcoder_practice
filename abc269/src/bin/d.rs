use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    // grid[x][y] = i
    let mut grid = vec![vec![-1i64; 2001]; 2001];
    for i in 0..n {
        let (x, y) = xy[i];
        // -1000 <= x, y <= 1000 なので +1000 して 0 <= x, y <= 2000 にする
        grid[(x + 1000) as usize][(y + 1000) as usize] = i as i64;
    }

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        let (x, y) = xy[i];
        // 隣接6方向
        let d = [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];
        for (dx, dy) in &d {
            let nx = x + 1000 + *dx;
            let ny = y + 1000 + *dy;
            if nx < 0 || ny < 0 || nx >= 2001 || ny >= 2001 {
                continue;
            }
            // 存在するなら unite
            if grid[nx as usize][ny as usize] != -1 {
                uf.unite(i, grid[nx as usize][ny as usize] as usize);
            }
        }
    }

    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(uf.find(i));
    }
    println!("{:?}", set.len());
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
