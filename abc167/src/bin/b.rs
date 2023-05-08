use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        mut k: i64,
    };

    if k <= a {
        println!("{}", k);
        return;
    }
    if k <= a + b {
        println!("{}", a);
        return;
    }
    println!("{}", a + (a + b - k));
}
