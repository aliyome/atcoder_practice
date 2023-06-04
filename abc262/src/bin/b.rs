use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m]
    };

    let mut edges = vec![vec![false; n]; n];
    for &(u, v) in &uv {
        edges[u - 1][v - 1] = true;
        edges[v - 1][u - 1] = true;
    }

    let mut ans = 0;
    for a in 0..n {
        for b in a..n {
            if a == b {
                continue;
            }
            for c in b..n {
                if a == c || b == c {
                    continue;
                }
                if edges[a][b] && edges[b][c] && edges[c][a] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
