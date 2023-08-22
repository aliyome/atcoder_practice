use proconio::input;

fn main() {
    input! {
        h: u128,
        w: u128,
    };
    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    if (h * w) % 2 != 0 {
        println!("{}", (h * w) / 2 + 1);
    } else {
        println!("{}", (h * w) / 2);
    }
}
