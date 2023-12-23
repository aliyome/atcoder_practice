use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; k],
    }

    // 欠けた靴下の集合
    let set = a.into_iter().collect::<std::collections::HashSet<isize>>();

    // 残っている靴下
    let mut left = vec![];
    for i in 1..=n {
        let i = i as isize;
        left.push(i);
        if !set.contains(&i) {
            left.push(i);
        }
    }

    // 先頭から順番にペアを作ってスコアを計算
    let mut score1 = vec![];
    for i in (1..left.len()).step_by(2) {
        score1.push((left[i] - left[i - 1]).abs());
    }

    // 末尾から順番にペアを作ってスコアを計算
    left.reverse();
    let mut score2 = vec![];
    for i in (1..left.len()).step_by(2) {
        score2.push((left[i] - left[i - 1]).abs());
    }

    // 累積和
    for i in 1..score1.len() {
        score1[i] += score1[i - 1];
    }
    for i in 1..score2.len() {
        score2[i] += score2[i - 1];
    }

    score2.reverse();
    eprintln!("{:?}", score1);
    eprintln!("{:?}", score2);

    score1.insert(0, 0);
    score2.push(0);

    // 偶数個の場合はそのまま出力
    if left.len() % 2 == 0 {
        println!("{}", score1.last().unwrap());
        return;
    }

    let mut ans = std::isize::MAX;
    for i in 0..score1.len() {
        ans = ans.min(score1[i] + score2[i]);
    }

    println!("{}", ans);
}
