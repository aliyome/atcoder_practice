use proconio::input;
use proconio::marker::Chars;

const MOD: i64 = 998244353;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    let mut count_red = 0;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // 上下左右

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '.' {
                count_red += 1;
            }
        }
    }

    let mut result = 0;

    // 各赤セルについて、それを緑に塗り替えたときの影響を考慮
    for i in 0..h {
        for j in 0..w {
            // 現在のセルが赤の場合
            if grid[i][j] == '.' {
                let mut connected_component_diff = 1; // このセル自体が新しい連結成分となる
                let mut seen = std::collections::HashSet::new();

                for (dx, dy) in dirs.iter() {
                    let new_i = i as i32 + dx;
                    let new_j = j as i32 + dy;

                    if new_i >= 0 && new_i < h as i32 && new_j >= 0 && new_j < w as i32 {
                        let new_i = new_i as usize;
                        let new_j = new_j as usize;

                        // 隣接するセルが緑色であり、まだ確認していない場合
                        if grid[new_i][new_j] == '#' && !seen.contains(&(new_i, new_j)) {
                            // この緑色のセルが既存の連結成分の一部である可能性を考慮
                            connected_component_diff -= 1;
                            seen.insert((new_i, new_j));
                        }
                    }
                }

                result = (result + connected_component_diff + MOD) % MOD;
            }
        }
    }

    // 全ての赤セルについての期待値を計算
    result = result * mod_pow(count_red, MOD - 2, MOD) % MOD;

    println!("{}", result);
}

// 累乗をMODをとりながら計算する関数
fn mod_pow(mut a: i64, mut n: i64, modu: i64) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % modu;
        }
        a = a * a % modu;
        n >>= 1;
    }
    res
}
