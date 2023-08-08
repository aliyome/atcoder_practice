use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        txy: [(usize, usize, usize); q],
    }

    let mut shift = 0;
    let mut a = a;
    for (t, x, y) in txy {
        if t == 1 {
            // 0-indexed
            let x = x - 1;
            let y = y - 1;
            let x = (x + n - shift) % n;
            let y = (y + n - shift) % n;
            a.swap(x, y);
        } else if t == 2 {
            shift = (shift + 1) % n;
        } else {
            // 0-indexed
            let x = x - 1;
            println!("{}", a[(x + shift) % n]);
        }
    }
}
