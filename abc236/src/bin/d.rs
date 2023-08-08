use proconio::input;

fn main() {
    input! {
        n: usize, // <= 8
    };

    let mut bit_point = vec![0; 1 << 2 * n];
    for i in 1..2 * n {
        input! {
            a: [isize; 2*n - i]
        }
        for j in 0..a.len() {
            bit_point[1 << (i - 1) | 1 << (i + j)] = a[j];
        }
    }

    // DEBUG
    // for i in 0..1 << 2 * n {
    //     println!("{:04b} {}", i, bit_point[i]);
    // }

    // dp[i][bit] := i組目までを使って、bitの人を選んだときの最大値
    let mut dp = vec![vec![0; 1 << 2 * n]; 2 * n + 1];
    for i in 0..n {
        // bit := 今までに選んだ人の集合
        for from_bit in 0..(1 << (2 * n)) as usize {
            // 今まで選ばれた人がi * 2人でなければスキップ (i組目までにはi * 2人選ばれている)
            if from_bit.count_ones() != (i * 2) as u32 {
                continue;
            }

            // 次に選ぶ人
            // FIXME
            for next_bit in 0..(1 << (2 * n)) as usize {
                // 今まで選ばれた人がi * 2人でなければスキップ (i組目までにはi * 2人選ばれている)
                let pair = from_bit ^ next_bit;
                if pair.count_ones() != 2 {
                    continue;
                }
                dp[i + 1][next_bit] = dp[i + 1][next_bit].max(dp[i][from_bit] + bit_point[pair]);
            }
        }
    }

    println!("{}", dp[n][(1 << (2 * n)) - 1]);
}
