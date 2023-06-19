use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    if format!("{:02}{:02}", a, b) <= format!("{:02}{:02}", c, d) {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
