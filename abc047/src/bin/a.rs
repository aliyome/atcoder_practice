use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };

    if a == b + c || b == a + c || c == a + b {
        println!("Yes");
    } else {
        println!("No");
    }
}
