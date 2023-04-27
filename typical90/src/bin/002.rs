use proconio::input;

fn main() {
    input! {
        n: usize
    };

    // 初期値は空文字列と文字列長
    solve("", n);
}

fn solve(s: &str, left: usize) {
    if left == 0 {
        if is_ok(s) {
            println!("{}", s);
        }
        return;
    }
    // 全探索
    solve(&format!("{}(", s), left - 1);
    solve(&format!("{})", s), left - 1);
}

fn is_ok(s: &str) -> bool {
    let mut cnt = 0;
    for c in s.chars() {
        if c == '(' {
            cnt += 1;
        } else {
            cnt -= 1;
        }
        if cnt < 0 {
            return false;
        }
    }
    cnt == 0
}
