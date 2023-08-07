use proconio::input;

fn main() {
    input! {
        x: char,
        y: char,
    };
    if x < y {
        println!("<");
    } else if x > y {
        println!(">");
    } else {
        println!("=");
    }
}
