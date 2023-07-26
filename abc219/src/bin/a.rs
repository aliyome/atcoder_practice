use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    for y in [40, 70, 90] {
        if x < y {
            println!("{}", y - x);
            return;
        }
    }

    println!("expert");
}
