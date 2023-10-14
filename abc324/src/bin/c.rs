use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
        s: [String; n],
    }

    let mut ans = vec![];

    for (idx, si) in s.iter().enumerate() {
        if check_possible(&t, si) {
            ans.push(idx + 1);
        }
    }

    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn check_possible(t: &String, s: &String) -> bool {
    // 条件1
    if t == s {
        return true;
    }

    // 条件2
    if t.len() + 1 == s.len() {
        for i in 0..=t.len() {
            if &t[0..i] == &s[0..i] && &t[i..] == &s[i + 1..] {
                return true;
            }
        }
    }

    // 条件3
    if t.len() == s.len() + 1 {
        for i in 0..=s.len() {
            if &t[0..i] == &s[0..i] && &t[i + 1..] == &s[i..] {
                return true;
            }
        }
    }

    // 条件4
    if t.len() == s.len() {
        let mut diff_count = 0usize;
        for (c_t, c_s) in t.chars().zip(s.chars()) {
            if c_t != c_s {
                diff_count += 1;
            }
        }
        if diff_count == 1 {
            return true;
        }
    }

    false
}
