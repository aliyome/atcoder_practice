use proconio::input;

fn main() {
    input! {
      d: usize, // <= 10^6
      n: usize, // <= 10^6
      lr: [(usize, usize); n]
    }

    let mut sum = vec![0; d + 1];
    for &(l, r) in &lr {
        for i in l..=r {
            sum[i] += 1;
        }
    }

    for d in 1..=d {
        println!("{}", sum[d]);
    }
}
