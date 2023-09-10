use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let ab = format!("{}{}", a, b).parse::<u32>().unwrap();
    println!("{}", ab * 2);
}
