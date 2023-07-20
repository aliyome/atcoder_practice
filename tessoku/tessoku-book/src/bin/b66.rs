use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m],
      q: usize,
    }

    let mut list = vec![];
    let mut stop = vec![false; m];
    for _ in 0..q {
        input! {
          t: usize
        }

        match t {
            1 => {
                input! {
                    x: usize
                }
                list.push((t, x, 0));
                stop[x - 1] = true;
            }
            2 => {
                input! {
                    u: usize,
                    v: usize,
                }
                list.push((t, u, v));
            }
            _ => unreachable!(),
        }
    }

    let mut uf = UnionFind::new(n + 1);

    // 最後まで残っていた路線を統合
    for i in 0..m {
        if !stop[i] {
            uf.unite(ab[i].0, ab[i].1);
        }
    }

    // UnionFindを逆順に作る
    list.reverse();
    let mut ans = vec![];
    for &(t, ux, v) in &list {
        if t == 1 {
            uf.unite(ab[ux - 1].0, ab[ux - 1].1);
        } else {
            ans.push(uf.same(ux, v));
        }
    }

    // 出力
    ans.reverse();
    for a in ans {
        if a {
            println!("Yes");
        } else {
            println!("No");
        }
    }
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
