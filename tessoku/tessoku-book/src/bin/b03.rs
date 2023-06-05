use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    for &i in &a {
        for &j in &a {
            for &k in &a {
                if i + j + k == 1000 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
