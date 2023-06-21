use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    let a = n / 3;
    let b = n / 5;
    let c = n / 7;

    let ab = n / (3 * 5);
    let bc = n / (5 * 7);
    let ca = n / (7 * 3);

    let abc = n / (3 * 5 * 7);

    let ans = a + b + c - (ab + bc + ca) + abc;

    println!("{}", ans);
}
