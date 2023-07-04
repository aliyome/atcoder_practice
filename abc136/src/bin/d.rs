use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();
    let mut r_max = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    while l < n {
        while r < n && s[l] == s[r] {
            r += 1;
        }
        r_max[l] = r - l;
        l += 1;
    }
    let mut l_max = vec![0; n];
    let mut l = n as isize - 1;
    let mut r = n as isize - 1;
    while 0 <= l {
        while 0 <= r && s[l as usize] == s[r as usize] {
            r -= 1;
        }
        l_max[l as usize] = (l - r) as usize;
        l -= 1;
    }

    let mut ans = vec![0isize; n];
    for i in 0..n {
        if s[i] == 'R' {
            if r_max[i] % 2 == 0 {
                ans[i + r_max[i]] += 1;
            } else {
                ans[i + r_max[i] - 1] += 1;
            }
        } else {
            if l_max[i] % 2 == 0 {
                ans[i - l_max[i]] += 1;
            } else {
                ans[i - l_max[i] + 1] += 1;
            }
        }
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }
}
