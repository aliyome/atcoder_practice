use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    if a == b - 1 && (a - 1) / 3 == (b - 1) / 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
