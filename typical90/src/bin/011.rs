use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        dcs: [(usize, usize, usize); n]
    }

    // //
    // // 全探索 O(N!) 2点
    // //
    // let mut max = 0;
    // // 全組み合わせを列挙
    // for indices in (0..n).permutations(n) {
    //     let mut sum = 0;
    //     let mut day = 0;
    //     for i in indices {
    //         let (d, c, s) = dcs[i];
    //         day += c;
    //         if day > d {
    //             break;
    //         }
    //         sum += s;
    //     }
    //     max = max.max(sum);
    // }
    // println!("{}", max);

    //     //
    //     // ビット全探索 O(2^N) 2点
    //     // 2^N は N が 20 くらいまでなら間に合う
    //     //
    //     // 締め切りでソート
    //     let mut dcs = dcs;
    //     dcs.sort_by_key(|&(d, _, _)| d);
    //     let calc_score = |mask| {
    //         let mut day = 0;
    //         let mut score: isize = 0;
    //         for i in 0..n {
    //             if mask & 1 << i == 0 {
    //                 continue;
    //             }
    //             if day + dcs[i].1 > dcs[i].0 {
    //                 return -1;
    //             }
    //             day += dcs[i].1;
    //             score += dcs[i].2 as isize;
    //         }
    //         score
    //     };

    //     let mut ans = 0;
    //     for i in 0..1 << n {
    //         let score = calc_score(i);
    //         ans = ans.max(score);
    //     }
    //     println!("{}", ans);

    //
    // DP O(DN) 2点
    //
    // 締め切りでソート
    let mut dcs = dcs;
    dcs.sort_by_key(|&(d, _, _)| d);

    // dp[i][j] := 報酬が高くなるように仕事 i を採用したりしなかったりした場合の j 日目までの報酬の最大値
    let mut dp = vec![vec![0; 5001]; n + 1];
    for i in 0..n {
        let (d, c, s) = dcs[i];
        for j in 0..=5000 {
            // 仕事 i を採用しない場合
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            if j + c <= d {
                // 仕事 i を採用する場合
                dp[i + 1][j + c] = (dp[i][j] + s).max(dp[i][j + c]);
            }
        }
    }

    let mut ans = 0;
    for i in 0..=5000 {
        ans = ans.max(dp[n][i]);
    }
    println!("{}", ans);
}
