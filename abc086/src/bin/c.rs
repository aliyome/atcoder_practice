use proconio::input;

fn main() {
    input! {
        n: usize,
        txy: [(isize, isize, isize); n]
    };

    let mut pt = 0;
    let mut px = 0;
    let mut py = 0;
    for &(t, x, y) in &txy {
        let dt = t - pt;
        let dx = (x - px).abs();
        let dy = (y - py).abs();
        let d = dx + dy;

        if dt < d {
            println!("No");
            return;
        }

        if (dt - d) % 2 != 0 {
            println!("No");
            return;
        }

        pt = t;
        px = x;
        py = y;
    }
    println!("Yes");
}
