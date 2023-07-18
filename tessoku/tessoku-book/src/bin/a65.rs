use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n - 1]
    }

    // indexを2から始めるために先頭にダミーを挿入
    a.insert(0, 0);
    a.insert(0, 0);

    // 木を作る
    let mut sub = vec![vec![]; n + 1];
    for i in 2..=n {
        sub[a[i]].push(i);
    }

    // DP
    let mut dp = vec![0usize; n + 1];
    for i in (1..=n).rev() {
        for &j in &sub[i] {
            dp[i] += dp[j] + 1;
        }
    }

    for i in 1..=n {
        print!("{} ", dp[i]);
    }
}

// ↓解説を読む前に BFS で解いたもの
// use std::collections::VecDeque;

// use proconio::input;

// fn main() {
//     input! {
//       n: usize,
//       mut a: [usize; n - 1]
//     }

//     // indexを2から始めるために先頭にダミーを挿入
//     a.insert(0, 0);
//     a.insert(0, 0);

//     // 木を作る
//     let mut to_child = vec![vec![]; n + 1];
//     let mut child_count = vec![0; n + 1]; // 各ノードが持つ未処理の子の数
//     for i in 2..=n {
//         to_child[a[i]].push(i);
//         child_count[a[i]] += 1;
//     }

//     // 葉を探す
//     let mut targets = VecDeque::new();
//     for i in 1..=n {
//         if to_child[i].len() == 0 {
//             targets.push_back(i);
//         }
//     }

//     // 葉から親に
//     let mut sub = vec![0; n + 1];
//     while let Some(i) = targets.pop_front() {
//         let parent = a[i];
//         sub[parent] += sub[i] + 1;
//         child_count[parent] -= 1;

//         // 全ての子から訪問された後にのみ、親をキューに追加
//         if parent != 0 && child_count[parent] == 0 {
//             targets.push_back(parent);
//         }
//     }

//     for i in 1..=n {
//         print!("{} ", sub[i]);
//     }
// }
