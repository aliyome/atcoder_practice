use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [usize; n],
        b: [usize; n]
    }

    // Grundy数を事前に計算する
    // W = B = 50 の時、1325 が B が取りうる最大値
    let max_w = 50;
    let max_b = 1350;
    let mut grundy = vec![vec![0; max_b + 1]; max_w + 1];
    for white in 0..=max_w {
        for blue in 0..=max_b {
            let mut reachable_mexes = std::collections::HashSet::new();
            // 到達可能な範囲のgrundy数
            for i in 1..=(blue / 2) {
                let b = blue - i;
                reachable_mexes.insert(grundy[white][b]);
            }
            // white を 1 つ減らした時に到達可能な位置の grundy 数
            if white > 0 && blue + white <= max_b {
                reachable_mexes.insert(grundy[white - 1][blue + white]);
            }
            // white, blue の mex = grundy数
            let mut mex = 0;
            for i in 0..max_b {
                if !reachable_mexes.contains(&i) {
                    mex = i;
                    break;
                }
            }
            grundy[white][blue] = mex;
        }
    }

    // 各山の Grundy 数の XOR を取る
    let mut xor = 0;
    for i in 0..n {
        xor ^= grundy[w[i]][b[i]];
    }

    // Grundy 数の XOR が 0 なら後手必勝
    if xor == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
