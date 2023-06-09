use proconio::input;

fn main() {
    input! {
        x: usize,
    };

    if x == 7 || x == 5 || x == 3 {
        println!("YES");
    } else {
        println!("NO");
    }
}
