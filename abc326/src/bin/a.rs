use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    if 0 <= y - x && y - x <= 2 {
        println!("Yes");
    } else if 0 <= x - y && x - y <= 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
