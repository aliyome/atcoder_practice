use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut red = vec![vec![false; w]; h];
    let mut uf = UnionFind::new(h * w);

    for _ in 0..q {
        input! {
            t: usize
        }
        if t == 1 {
            input! {
                r: usize,
                c: usize,
            }
            let r = r - 1;
            let c = c - 1;
            red[r][c] = true;
            for (vx, vy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let x = r as i32 + vx;
                let y = c as i32 + vy;
                if x < 0 || x >= h as i32 || y < 0 || y >= w as i32 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if red[x][y] {
                    uf.union(r * w + c, x * w + y);
                }
            }
        } else {
            input! {
                ra: usize,
                ca: usize,
                rb: usize,
                cb: usize,
            }
            let ra = ra - 1;
            let ca = ca - 1;
            let rb = rb - 1;
            let cb = cb - 1;
            if uf.equiv(ra * w + ca, rb * w + cb) && red[ra][ca] && red[rb][cb] {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
