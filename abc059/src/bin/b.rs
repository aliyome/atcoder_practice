use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
    };

    let mut aa = a.clone();
    let mut bb = b.clone();
    if a.len() < b.len() {
        aa = vec![];
        for _ in 0..b.len() - a.len() {
            aa.push('0');
        }
        for c in a {
            aa.push(c);
        }
    } else if a.len() > b.len() {
        bb = vec![];
        for _ in 0..a.len() - b.len() {
            bb.push('0');
        }
        for c in b {
            bb.push(c);
        }
    }

    if aa < bb {
        println!("LESS");
    } else if aa > bb {
        println!("GREATER");
    } else {
        println!("EQUAL");
    }
}
