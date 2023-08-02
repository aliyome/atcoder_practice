use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      lr: [(usize, usize); n]
    }

    // 貪欲法で区間スケジューリング
    // r で昇順ソート
    let mut sorted = lr.clone();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));
    // left[r] := r 未満の区間に含まれる仕事の数
    let mut left = vec![0usize; 200000];
    let mut t = 0;
    for &(l, r) in &sorted {
        // 開始時間を過ぎている場合は無視
        if t > 0 && t + k > l {
            continue;
        }
        t = r;
        left[t] += 1;
    }
    // 累積和
    for i in 1..left.len() {
        left[i] += left[i - 1];
    }

    // 同様に右から
    // l で降順ソート
    let mut sorted = lr.clone();
    sorted.sort_by(|a, b| b.0.cmp(&a.0));
    // right[l] := l 以上の区間に含まれる仕事の数
    let mut right = vec![0usize; 200000];
    let mut t = right.len() - 1;
    for &(l, r) in &sorted {
        if t == right.len() - 1 || r <= t - k {
            t = l;
            right[t] += 1;
        }
    }
    // 累積和
    for i in (1..right.len() - 1).rev() {
        right[i] += right[i + 1];
    }

    // 出力
    for &(l, r) in &lr {
        let a = if l < k { 0 } else { left[l - k] };
        let b = right[r + k];
        println!("{}", a + 1 + b);
    }
}
