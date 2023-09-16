use proconio::input;
use proconio::marker::Usize1;

#[derive(Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Diff {
    dx: i64,
    dy: i64,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    pos_diff: Vec<Diff>,
    root: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            pos_diff: vec![Diff { dx: 0, dy: 0 }; n],
            root: (0..n).collect(),
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.parent[u] == u {
            u
        } else {
            let parent = self.parent[u];
            self.parent[u] = self.find(parent);
            let diff = self.pos_diff[parent];
            self.pos_diff[u].dx += diff.dx;
            self.pos_diff[u].dy += diff.dy;
            self.root[u] = self.root[parent];
            self.parent[u]
        }
    }

    fn unite(&mut self, u: usize, v: usize, dx: i64, dy: i64) {
        let pu = self.find(u);
        let pv = self.find(v);

        if pu == pv {
            return;
        }

        let diff_x = dx + self.pos_diff[u].dx - self.pos_diff[v].dx;
        let diff_y = dy + self.pos_diff[u].dy - self.pos_diff[v].dy;

        if self.rank[pu] < self.rank[pv] {
            self.parent[pu] = pv;
            self.pos_diff[pu].dx = -diff_x;
            self.pos_diff[pu].dy = -diff_y;
            self.root[pu] = self.root[pv];
        } else {
            self.parent[pv] = pu;
            self.pos_diff[pv].dx = diff_x;
            self.pos_diff[pv].dy = diff_y;
            self.root[pv] = self.root[pu];

            if self.rank[pu] == self.rank[pv] {
                self.rank[pu] += 1;
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        info: [(Usize1, Usize1, i64, i64); m],
    }

    let mut uf = UnionFind::new(n);

    for (ai, bi, xi, yi) in info {
        uf.unite(ai, bi, xi, yi);
    }

    let mut coords = vec![Pos { x: 0, y: 0 }; n];
    for i in 0..n {
        uf.find(i);
        coords[i].x = uf.pos_diff[i].dx;
        coords[i].y = uf.pos_diff[i].dy;
    }

    let offset_x = coords[0].x;
    let offset_y = coords[0].y;

    for i in 0..n {
        if uf.root[i] != uf.root[0] {
            println!("undecidable");
        } else {
            println!("{} {}", coords[i].x - offset_x, coords[i].y - offset_y);
        }
    }
}
