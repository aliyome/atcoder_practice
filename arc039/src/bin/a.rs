use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
    };

    let mut aa = a.clone();
    let mut bb = b.clone();
    let mut aaa = a.clone();
    let mut bbb = b.clone();

    aa[0] = '9';
    bb[0] = '9';
    aaa[0] = '1';
    bbb[0] = '1';

    let a = a.iter().collect::<String>().parse::<i32>().unwrap();
    let aa = aa.iter().collect::<String>().parse::<i32>().unwrap();
    let aaa = aaa.iter().collect::<String>().parse::<i32>().unwrap();
    let b = b.iter().collect::<String>().parse::<i32>().unwrap();
    let bb = bb.iter().collect::<String>().parse::<i32>().unwrap();
    let bbb = bbb.iter().collect::<String>().parse::<i32>().unwrap();

    let ans = (a - b).max(a - bb).max(a - bbb).max(aa - b).max(aaa - b);
    println!("{}", ans);
}
