use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
    };

    let mut aa = vec![];
    let mut bb = vec![];
    for i in 0..3 {
        let mut a = a.clone();
        let mut b = b.clone();
        let alen = a.len();
        let blen = b.len();
        if alen > i {
            a[i] = '9';
            aa.push(a.iter().collect::<String>().parse::<i32>().unwrap());
            if i == 0 {
                a[i] = '1';
                aa.push(a.iter().collect::<String>().parse::<i32>().unwrap());
            } else {
                a[i] = '0';
                aa.push(a.iter().collect::<String>().parse::<i32>().unwrap());
            }
        }

        if blen > i {
            b[i] = '9';
            bb.push(b.iter().collect::<String>().parse::<i32>().unwrap());
            if i == 0 {
                b[i] = '1';
                bb.push(b.iter().collect::<String>().parse::<i32>().unwrap());
            } else {
                b[i] = '0';
                bb.push(b.iter().collect::<String>().parse::<i32>().unwrap());
            }
        }
    }

    let a = a.iter().collect::<String>().parse::<i32>().unwrap();
    let b = b.iter().collect::<String>().parse::<i32>().unwrap();
    let mut ans = -10000;
    for b in bb {
        ans = ans.max(a - b);
    }
    for a in aa {
        ans = ans.max(a - b);
    }
    println!("{}", ans);
}
