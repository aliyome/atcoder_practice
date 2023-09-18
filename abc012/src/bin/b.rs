use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let s = n % 60;
    let m = n / 60;
    let h = m / 60;
    let m = m % 60;

    println!("{:02}:{:02}:{:02}", h, m, s);
}
