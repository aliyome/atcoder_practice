use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let mut aa = vec![];
    let mut bb = vec![];
    for _ in 0..b {
        aa.push(a);
    }
    for _ in 0..a {
        bb.push(b);
    }

    if aa < bb {
        println!("{}", aa.iter().map(|x| x.to_string()).collect::<String>());
    } else {
        println!("{}", bb.iter().map(|x| x.to_string()).collect::<String>());
    }
}
