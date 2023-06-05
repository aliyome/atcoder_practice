use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        c: usize,
    };

    if a < b {
        println!("Aoki");
    } else if a > b {
        println!("Takahashi");
    } else {
        if c == 0 {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    }
}
