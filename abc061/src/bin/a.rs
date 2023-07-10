use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    };

    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
