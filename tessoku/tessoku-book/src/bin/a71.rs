use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n],
      mut b: [usize; n]
    }

    a.sort();
    b.sort();
    b.reverse();

    let mut cost = 0;
    for i in 0..n {
        cost += a[i] * b[i];
    }

    println!("{}", cost);
}
