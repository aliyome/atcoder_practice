use proconio::input;

fn main() {
    input! {
        w: f64,
        h: f64,
        x: f64,
        y: f64,
    };

    if x == w / 2.0 && y == h / 2.0 {
        println!("{} {}", w * h / 2.0, 1);
    } else {
        println!("{} {}", w * h / 2.0, 0);
    }
}
