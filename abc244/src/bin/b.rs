use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String
    };

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut vx: i64 = 1;
    let mut vy: i64 = 0;

    for c in t.chars() {
        match c {
            'S' => {
                x += vx;
                y += vy;
            }
            'R' => {
                let tmp = vx;
                vx = vy;
                vy = -tmp
            }
            _ => unreachable!(),
        }
    }

    println!("{} {}", x, y);
}
