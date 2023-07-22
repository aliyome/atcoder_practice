use proconio::input;

fn main() {
    input! {
      n: usize,
      p: [[usize; n]; n]
    }

    let mut row = vec![0; n];
    let mut col = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            let d = p[i][j];
            if d != 0 {
                row[d - 1] = i;
                col[d - 1] = j
            }
        }
    }

    let mut inversion_count = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if row[i] > row[j] {
                inversion_count += 1;
            }
        }
        for j in i + 1..n {
            if col[i] > col[j] {
                inversion_count += 1;
            }
        }
    }

    println!("{}", inversion_count);
}
