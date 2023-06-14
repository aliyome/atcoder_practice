use proconio::input;

fn main() {
    input! {
        n: i128,
    };

    if -2i128.pow(31) <= n && n < 2i128.pow(31) {
        println!("Yes");
    } else {
        println!("No");
    }
}
