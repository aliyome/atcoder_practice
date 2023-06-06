use proconio::input;

fn main() {
    input! {
      t: usize, // 5x10^5
      n: usize, // 5x10^5
      lr: [(usize, usize); n]
    }

    // いもす法で累積和を計算する
    let mut counts = vec![0isize; t + 2];
    for &(l, r) in &lr {
        counts[l] += 1;
        counts[r] -= 1;
    }
    for i in 0..t {
        counts[i + 1] += counts[i];
        println!("{}", counts[i]);
    }
}
