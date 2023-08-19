use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let parts = [
        ["k", "eyence"],
        ["ke", "yence"],
        ["key", "ence"],
        ["keye", "nce"],
        ["keyen", "ce"],
        ["keyenc", "e"],
        ["keyence", ""],
    ];
    let mut list = vec![];
    for i in 0..s.len() {
        for j in i..=s.len() {
            for &[p1, p2] in &parts {
                if s[i..j] == p1.chars().collect::<Vec<char>>() {
                    list.push((i, j, p2));
                }
            }
        }
    }
    // println!("{:?}", list);

    for i in 0..s.len() {
        for j in i..=s.len() {
            for &(_, jj, p2) in &list {
                if i < jj {
                    continue;
                }

                // println!("{} {} {}", i, j, s[i..j].iter().collect::<String>());
                if s[i..j] == p2.chars().collect::<Vec<char>>() {
                    println!("YES");
                    return;
                }
            }
        }
    }

    println!("NO");
}
