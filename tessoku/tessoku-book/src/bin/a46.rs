use rand::Rng;
use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
      n: usize,
      xy: [(isize, isize); n]
    }

    let calc_dist = |(x1, y1): (isize, isize), (x2, y2): (isize, isize)| {
        (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
    };

    let calc_score = |ans: &Vec<usize>| {
        let mut score = 0.0;
        for i in 0..n {
            score += calc_dist(xy[ans[i]], xy[ans[i + 1]]);
        }
        score
    };

    // // 2-opt 局所探索法
    // let mut rng = rand::thread_rng();
    // let mut ans = (0..n).collect::<Vec<_>>();
    // ans.push(0);

    // for _ in 0..200000 {
    //     let l = rng.gen_range(1, n);
    //     let r = rng.gen_range(1, n);
    //     // l..r の区間を反転させる
    //     if l > r {
    //         continue;
    //     }
    //     let mut tmp = ans[l..r].to_vec();
    //     tmp.reverse();
    //     let mut new_ans = ans[..l].to_vec();
    //     new_ans.append(&mut tmp);
    //     new_ans.append(&mut ans[r..].to_vec());
    //     let score = calc_score(&ans);
    //     let new_score = calc_score(&new_ans);
    //     if new_score < score {
    //         ans = new_ans;
    //     }
    // }

    // for &a in &ans {
    //     println!("{} ", a + 1);
    // }

    // // 最近傍
    // let mut q = VecDeque::new();
    // q.push_back(xy[0]);
    // let mut used = vec![false; n];

    // let mut ans = vec![0];
    // while let Some((x, y)) = q.pop_front() {
    //     let mut next = 0;
    //     let mut dist = std::isize::MAX;
    //     for j in 1..n {
    //         if used[j] {
    //             continue;
    //         }
    //         let (x2, y2) = xy[j];
    //         let d = (x2 - x).pow(2) + (y2 - y).pow(2);
    //         if d < dist {
    //             dist = d;
    //             next = j;
    //         }
    //     }

    //     if next == 0 {
    //         break;
    //     }

    //     used[next] = true;
    //     ans.push(next);
    //     q.push_back(xy[next]);
    // }
    // for &a in &ans {
    //     println!("{} ", a + 1);
    // }
    // println!("1");
}
