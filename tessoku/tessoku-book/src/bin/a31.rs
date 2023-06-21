use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    let a = n / 3;
    let b = n / 5;
    let c = n / (3 * 5);
    let ans = a + b - c;

    println!("{}", ans);
}
