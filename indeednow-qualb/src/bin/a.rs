use proconio::input;

fn main() {
    input! {
        xy1: (isize, isize),
        xy2: (isize, isize),
    };

    println!("{}", (xy1.0 - xy2.0).abs() + (xy1.1 - xy2.1).abs() + 1);
}
