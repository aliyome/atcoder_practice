use proconio::input;

fn main() {
    input! {
        b: u128,
    }

    for a in 1u128.. {
        let x: u128 = a.pow(a as u32);
        if x > 10u128.pow(18) {
            break;
        }
        if x == b {
            println!("{}", a);
            return;
        }
    }

    println!("-1");
}
