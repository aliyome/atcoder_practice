use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    };

    if b % a == 0 {
        println!("{}", a + b);
    } else {
        println!("{}", b - a);
    }
}
