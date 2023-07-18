use proconio::{input, marker::Chars};

fn main() {
    input! {
        abc: Chars,
    };

    let x: usize = format!("{}{}{}", abc[0], abc[1], abc[2]).parse().unwrap();
    let y: usize = format!("{}{}{}", abc[1], abc[2], abc[0]).parse().unwrap();
    let z: usize = format!("{}{}{}", abc[2], abc[0], abc[1]).parse().unwrap();
    println!("{}", x + y + z);
}
