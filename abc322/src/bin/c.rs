use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        a: [usize; m]
    }

    let mut day = 1;
    for &a in &a {
        while day <= a {
            println!("{}", a - day);
            day += 1;
        }
    }
}
