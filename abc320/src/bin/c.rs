use proconio::input;

fn main() {
    input! {
        m: usize,
        s1: String,
        s2: String,
        s3: String,
    }
    let s1: Vec<_> = s1.chars().collect();
    let s2: Vec<_> = s2.chars().collect();
    let s3: Vec<_> = s3.chars().collect();

    let mut ans = std::usize::MAX;

    for t1 in 0..m {
        for t2 in t1 + 1..m * 2 {
            for t3 in t2 + 1..m * 3 {
                if s1[t1] == s2[t2 % m] && s2[t2 % m] == s3[t3 % m] {
                    let time = t1.max(t2).max(t3);
                    ans = ans.min(time);
                }
                if s1[t1] == s3[t2 % m] && s3[t2 % m] == s2[t3 % m] {
                    let time = t1.max(t2).max(t3);
                    ans = ans.min(time);
                }
                if s2[t1] == s1[t2 % m] && s1[t2 % m] == s3[t3 % m] {
                    let time = t1.max(t2).max(t3);
                    ans = ans.min(time);
                }
                if s2[t1] == s3[t2 % m] && s3[t2 % m] == s1[t3 % m] {
                    let time = t1.max(t2).max(t3);
                    ans = ans.min(time);
                }
                if s3[t1] == s1[t2 % m] && s1[t2 % m] == s2[t3 % m] {
                    let time = t1.max(t2).max(t3);
                    ans = ans.min(time);
                }
                if s3[t1] == s2[t2 % m] && s2[t2 % m] == s1[t3 % m] {
                    let time = t1.max(t2).max(t3);
                    ans = ans.min(time);
                }
            }
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
