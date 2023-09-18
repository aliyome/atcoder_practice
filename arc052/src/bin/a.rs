use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ss = vec![];
    let mut standing = false;
    for i in 0..s.len() {
        let c = s[i];

        if standing && !c.is_digit(10) {
            break;
        }

        if c.is_digit(10) {
            ss.push(c);
            standing = true;
        }
    }
    println!("{}", ss.iter().collect::<String>().parse::<u32>().unwrap());
}
