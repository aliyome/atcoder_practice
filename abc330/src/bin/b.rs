use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        r: i32,
        a: [i32; n],
    }

    for &ai in &a {
        let xi = if ai < l {
            l
        } else if ai > r {
            r
        } else {
            ai
        };

        print!("{} ", xi);
    }
    println!();
}
