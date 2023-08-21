use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };
    println!("{}", a * d - b * c);
}
