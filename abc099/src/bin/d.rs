use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        d: [[usize; c]; c],
        grid: [[usize; n]; n]
    }

    let mut cost = vec![vec![0; c]; 3];

    for i in 0..n {
        for j in 0..n {
            for k in 0..c {
                cost[(i + j) % 3][k] += d[grid[i][j] - 1][k];
            }
        }
    }

    let mut ans = std::usize::MAX;

    for x in 0..c {
        for y in 0..c {
            for z in 0..c {
                if x == y || y == z || z == x {
                    continue;
                }
                ans = std::cmp::min(ans, cost[0][x] + cost[1][y] + cost[2][z]);
            }
        }
    }

    println!("{}", ans);
}
