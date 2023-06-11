use proconio::input;

fn main() {
    input! {
        x: isize,
        a: isize,
        b: isize,
    };

    let ans = (x - a) % b;

    println!("{}", ans);
}
