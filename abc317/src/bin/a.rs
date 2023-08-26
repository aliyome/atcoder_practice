use proconio::input;

fn main() {
    input! {
        n: usize,
        h: i32,
        x: i32,
        p: [i32; n]
    }

    for i in 0..n {
        if h + p[i] >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
