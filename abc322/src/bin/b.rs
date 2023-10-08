use proconio::input; // proconioクレートを使います。

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }

    let is_head = s == t[0..n];
    let is_tail = s == t[m - n..m];
    if is_head && is_tail {
        println!("0");
    } else if is_head {
        println!("1");
    } else if is_tail {
        println!("2");
    } else {
        println!("3");
    }
}
