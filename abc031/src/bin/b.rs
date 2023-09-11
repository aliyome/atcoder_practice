use proconio::input;

fn main() {
    input! {
        l: usize,
        h: usize,
        n: usize,
        a: [usize; n]
    };

    for &a in &a {
        if a < l {
            println!("{}", l - a);
        } else if a <= h {
            println!("0")
        } else {
            println!("-1");
        }
    }
}
