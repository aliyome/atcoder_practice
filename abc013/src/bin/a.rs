use proconio::input;

fn main() {
    input! {
        x: char,
    };

    println!("{}", (x as usize - b'A' as usize + 1));
}
