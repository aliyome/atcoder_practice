use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        q: isize,
        a: [usize; q]
    };

    let mut score = vec![0isize; n];
    for i in 0..q {
        score[a[i as usize] - 1] += 1;
    }

    // 全q問なので0問正解の場合は-q点
    // 1問正解すると1-(q-1)点
    // i問正解するとscore[i]-(q-score[i])点
    // この点数が k 以上なら生き残り
    for i in 0..n {
        let s = q - score[i];
        if k > s {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
