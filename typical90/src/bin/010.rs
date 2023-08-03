use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut acc = vec![vec![0; n + 1]; 2];
    for i in 0..n {
        acc[0][i + 1] = acc[0][i];
        acc[1][i + 1] = acc[1][i];

        let (c, p) = cp[i];
        let c = c - 1;
        acc[c][i + 1] += p;
    }

    for &(l, r) in &lr {
        for c in 0..2 {
            print!("{} ", acc[c][r] - acc[c][l - 1]);
        }
        println!();
    }
}
