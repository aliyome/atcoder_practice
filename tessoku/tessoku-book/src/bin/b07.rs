use proconio::input;

fn main() {
    input! {
      t: usize, // 5x10^5
      n: usize, // 5x10^5
      lr: [(usize, usize); n]
    }

    let mut counts = vec![0; t + 1];
    for &(l, r) in &lr {
        for i in l..r {
            counts[i] += 1;
        }
        for i in r..t {
            counts[i] += 0;
        }
    }
    for i in 0..t {
        println!("{}", counts[i]);
    }
}
