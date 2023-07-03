use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      mut a: [usize; m]
    }

    // 1-indexed
    a.insert(0, 0);

    let mut wrong = vec![0; n + 1];
    for &a in &a {
        wrong[a] += 1;
    }

    for i in 1..=n {
        println!("{}", m - wrong[i]);
    }
}
