use proconio::input;

fn main() {
    input! {
        M: usize,
        D: usize,
        y: usize,
        m: usize,
        d: usize,
    };

    if d == D {
        // 翌月
        if m == M {
            // 翌年
            println!("{} {} {}", y + 1, 1, 1);
        } else {
            println!("{} {} {}", y, m + 1, 1);
        }
    } else {
        println!("{} {} {}", y, m, d + 1);
    }
}
