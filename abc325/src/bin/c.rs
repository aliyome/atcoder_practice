use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut uf = UnionFind::new(h * w);

    // 各センサーに対して、隣接するセンサーとのUnion操作を行う
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                for &dir in &[(0, 1), (1, 0), (1, 1), (1, -1)] {
                    let ni = i as isize + dir.0;
                    let nj = j as isize + dir.1;

                    if ni >= 0
                        && ni < h as isize
                        && nj >= 0
                        && nj < w as isize
                        && s[ni as usize][nj as usize] == '#'
                    {
                        let u = i * w + j;
                        let v = ni as usize * w + nj as usize;
                        uf.unite(u, v);
                    }
                }
            }
        }
    }

    // グループ数をカウント
    let mut set = std::collections::HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                set.insert(uf.find(i * w + j));
            }
        }
    }

    println!("{}", set.len());
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
