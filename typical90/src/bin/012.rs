use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    // マス目
    let mut grid = vec![vec![false; w]; h];
    // Union-Find
    let mut union = UnionFind::new(h * w);

    // 2次元座標を1次元座標に変換する
    let conv_2d_to_1d = |x: usize, y: usize| -> usize { y * w + x };

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            // 塗る
            1 => {
                input! {
                    r: usize,
                    c: usize,
                }
                // 1オリジンを0オリジンに補正
                let r = r - 1;
                let c = c - 1;
                // 塗る
                grid[r][c] = true;

                // 4方向に対して、塗られているかを確認する

                for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
                    let x = c as i32 + *dx;
                    let y = r as i32 + *dy;
                    // 範囲外チェック
                    if x < 0 || w as i32 <= x || y < 0 || h as i32 <= y {
                        continue;
                    }
                    // 塗られていたら Union-Find を更新
                    if grid[y as usize][x as usize] {
                        let a = conv_2d_to_1d(c, r);
                        let b = conv_2d_to_1d(x as usize, y as usize);
                        union.unite(a, b);
                    }
                }
            }
            // たどる
            2 => {
                input! {
                    ra: usize,
                    ca: usize,
                    rb: usize,
                    cb: usize,
                }
                // 1オリジンを0オリジンに補正
                let ra = ra - 1;
                let ca = ca - 1;
                let rb = rb - 1;
                let cb = cb - 1;
                // 始点と終点が塗られているか
                if !grid[ra][ca] || !grid[rb][cb] {
                    println!("No");
                    continue;
                }
                // 始点と終点が同じか
                if ra == rb && ca == cb {
                    println!("Yes");
                    continue;
                }
                // 始点と終点が連結しているか
                let a = conv_2d_to_1d(ca, ra);
                let b = conv_2d_to_1d(cb, rb);
                if union.same(a, b) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => unreachable!(),
        }
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
            p
        }
    }

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

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
