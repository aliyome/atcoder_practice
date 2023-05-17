use proconio::input;

// 典型90問 020 Log Inequality
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    if a < c.pow(b as u32) {
        println!("Yes");
    } else {
        println!("No");
    }
}
