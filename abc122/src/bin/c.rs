use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
        lr: [(usize, usize); q]
    };

    let mut ac_counts: Vec<usize> = vec![0; s.len() + 1];
    for i in 0..(s.len() - 1) {
        ac_counts[i + 1] = ac_counts[i];
        if &s[i..=i + 1] == "AC" {
            ac_counts[i + 1] += 1;
        }
    }

    for i in 0..q {
        let l = lr[i].0 - 1;
        let r = lr[i].1 - 1;
        println!("{}", ac_counts[r] - ac_counts[l]);
    }
}
