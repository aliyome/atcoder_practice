use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: Chars,
    };
    if n.len() == 1 {
        println!("{}", n[0]);
        return;
    }

    let mut x = vec![];
    for i in 0..n.len() {
        if i == n.len() - 1 {
            x.push(n[i]);
        } else if n[i] == '9' {
            x.push('9');
        } else {
            let d = n[i].to_digit(10).unwrap() - 1;
            x.push(d.to_string().chars().next().unwrap());
            for _ in i + 1..n.len() {
                x.push('9');
            }
            break;
        }
    }

    let mut y = vec![];
    let d = n[0].to_digit(10).unwrap() - 1;
    y.push(d.to_string().chars().next().unwrap());
    for _ in 1..n.len() {
        y.push('9');
    }

    let x = x
        .into_iter()
        .map(|a| a.to_digit(10).unwrap() as usize)
        .sum::<usize>();
    let y = y
        .into_iter()
        .map(|a| a.to_digit(10).unwrap() as usize)
        .sum::<usize>();
    println!("{}", x.max(y));
}
