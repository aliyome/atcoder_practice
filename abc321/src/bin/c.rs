use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut ans = vec![];
    for i in 0..1 << 10 {
        let mut x = 0;
        for j in (0..=9).rev() {
            if i >> j & 1 == 1 {
                x = x * 10 + j;
            }
        }
        if x == 0 {
            continue;
        }
        ans.push(x);
    }

    ans.sort();
    println!("{}", ans[k - 1]);
}
