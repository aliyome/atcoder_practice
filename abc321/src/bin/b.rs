use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n - 1],
    }

    // 0~100のスコアを全探索
    let mut ans = std::usize::MAX;
    for last in 0..=100 {
        let mut all_scores = a.clone();
        all_scores.push(last);
        all_scores.sort_unstable();

        let sum = all_scores[1..n - 1].iter().sum::<usize>();

        if sum >= x {
            ans = ans.min(last);
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
