use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: isize,
        b: isize,
        c: [isize; m]
    };

    let mut n = n as isize;

    for i in 0..m {
        if n <= a {
            n += b;
        }
        n -= c[i];
        if n < 0 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("complete");
}
