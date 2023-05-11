use proconio::input;

fn main() {
    input! {
        n: f64,
    };

    if (1.08 * n).floor() < 206.0 {
        println!("Yay!");
    } else if (1.08 * n).floor() == 206.0 {
        println!("so-so");
    } else {
        println!(":(");
    }
}
