use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [usize; n]
    };

    // 1-indexed
    h.insert(0, 0);

    let mut ans = 0;
    loop {
        // 連結成分をリストのスタックとして管理する
        let mut stack = vec![];

        for i in 1..=n {
            if h[i] == 0 {
                continue;
            }

            if h[i - 1] == 0 {
                stack.push(vec![i]);
                continue;
            } else {
                stack.last_mut().unwrap().push(i);
            }
        }

        if stack.len() == 0 {
            break;
        }

        for i in 0..stack.len() {
            for &j in stack[i].iter() {
                h[j] -= 1;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
