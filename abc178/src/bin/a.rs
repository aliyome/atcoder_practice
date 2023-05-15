use proconio::input;

fn main() {
    input! {
        x: usize,
    };

    if x == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
