use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [[usize; n]; n],
      q: usize,
      queries: [(usize, usize, usize); q]
    }

    let mut rows = vec![0; n];
    for i in 0..n {
        rows[i] = i;
    }

    for &(t, x, y) in &queries {
        if t == 1 {
            let tmp = rows[x - 1];
            rows[x - 1] = rows[y - 1];
            rows[y - 1] = tmp;
        } else {
            println!("{}", a[rows[x - 1]][y - 1]);
        }
    }
}
