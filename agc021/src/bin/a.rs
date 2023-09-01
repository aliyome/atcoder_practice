use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };

    let n = n
        .into_iter()
        .map(|a| a.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    if n.len() == 1 {
        println!("{}", n[0]);
        return;
    }

    let mut candidates = vec![];
    candidates.push(n.clone());

    // i文字目を-1してi+1文字目以降を9にする
    for i in 0..n.len() {
        let mut x = vec![];
        for j in 0..n.len() {
            if j < i {
                x.push(n[j]);
            } else if j == i && n[j] != 0 {
                x.push(n[j] - 1);
                for _ in j + 1..n.len() {
                    x.push(9);
                }
                break;
            }
        }
        if x.len() == n.len() {
            candidates.push(x);
        }
    }

    let mut ans = 0;
    for c in candidates {
        ans = ans.max(c.iter().sum());
    }
    println!("{}", ans);
}
