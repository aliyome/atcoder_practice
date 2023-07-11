use proconio::input;

fn main() {
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize
    };

    if v * t <= d && d <= v * s {
        println!("No");
    } else {
        println!("Yes");
    }
}
