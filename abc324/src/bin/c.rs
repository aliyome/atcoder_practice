use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
        s: [String; n],
    }

    let ok = |a: &str, b: &str| {
        if a == b {
            return true;
        }

        if a.len() + 1 == b.len() {
            for i in 0..b.len() {
                if a[..i] == b[..i] && a[i..] == b[i + 1..] {
                    return true;
                }
            }
        }

        if a.len() == b.len() + 1 {
            for i in 0..a.len() {
                if a[..i] == b[..i] && a[i + 1..] == b[i..] {
                    return true;
                }
            }
        }

        if a.len() == b.len() {
            let mut diff = 0usize;
            for (x, y) in a.chars().zip(b.chars()) {
                if x != y {
                    diff += 1;
                }
            }
            if diff == 1 {
                return true;
            }
        }

        false
    };

    let mut ans = vec![];
    for i in 0..n {
        if ok(&t, &s[i]) {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
