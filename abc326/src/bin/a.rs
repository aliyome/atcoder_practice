use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    let diff = (x - y).abs();

    if diff <= 2 || (diff <= 3 && (x > y)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
