use proconio::input;

fn main() {
    input! {
        mut x: u128,
        y: u128,
        a: u128,
        b: u128,
    };

    let mut exp = 0;

    loop {
        let xx = x * a;
        if xx > x + b {
            break;
        }
        if xx >= y {
            break;
        }

        exp += 1;
        x = xx;
    }

    exp += (y - x - 1) / b;

    println!("{}", exp);
}
