use proconio::input;

fn main() {
    input! {
        m: usize,
        h: usize,
    };

    if h % m == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
