use proconio::input;

fn main() {
    input! {
        w: isize,
        h: isize,
        n: usize,
        xya: [(isize, isize, isize); n]
    };

    let mut xs = 0;
    let mut xe = w;
    let mut ys = 0;
    let mut ye = h;

    for &(x, y, a) in &xya {
        match a {
            1 => {
                xs = xs.max(x);
            }
            2 => {
                xe = xe.min(x);
            }
            3 => {
                ys = ys.max(y);
            }
            4 => {
                ye = ye.min(y);
            }
            _ => unreachable!(),
        }
    }

    println!("{}", (xe - xs).max(0) * (ye - ys).max(0));
}
