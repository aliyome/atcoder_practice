use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        query: [(usize, usize); q]
    };

    // // 愚直 (TLE)
    // let mut s = s;
    // for (t, x) in query {
    //     if t == 1 {
    //         let mut sub = s[s.len() - x..].to_vec();
    //         sub.extend(s[..s.len() - x].to_vec());
    //         // println!("s:{:?} sub:{:?}", s, sub);
    //         s = sub;
    //     } else {
    //         println!("{}", s[x - 1]);
    //     }
    // }

    // 位置だけ
    let mut start = 0;
    for (t, x) in query {
        if t == 1 {
            start = (start + s.len() - x) % n;
        } else {
            println!("{}", s[(start + x - 1) % n]);
        }
    }
}
