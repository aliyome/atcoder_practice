use proconio::input;
use proconio::marker::Usize1;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px); // Path compression
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.find(x);
        let mut y_root = self.find(y);
        if x_root == y_root {
            return false;
        }
        if self.size[x_root] < self.size[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }
        self.parent[y_root] = x_root;
        self.size[x_root] += self.size[y_root];
        true
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    }

    let mut uf = UnionFind::new(n * 2); // 各数字とその補数についてのグループを作る

    for i in 0..m {
        let (ai, bi) = (a[i], b[i]);

        // Ai と Bi のグループが同じかどうかチェックする
        if uf.same(ai, bi) {
            println!("No");
            return;
        }
        // Ai と補数 Bi、Bi と補数 Ai のグループを統合する
        uf.unite(ai, bi + n);
        uf.unite(ai + n, bi);
    }

    println!("Yes");
}
