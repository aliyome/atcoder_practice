use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }

    let mut cnt = 0;
    for a in a {
        if a >= l {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
