use proconio::input; // proconioクレートを使います。

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }

    // SがTのprefixかどうかをチェックします。
    let is_prefix = &t[0..n] == s;
    // SがTのsuffixかどうかをチェックします。
    let is_suffix = &t[(m - n)..] == s;

    // 条件に基づいて結果を出力します。
    if is_prefix && is_suffix {
        println!("0");
    } else if is_prefix {
        println!("1");
    } else if is_suffix {
        println!("2");
    } else {
        println!("3");
    }
}
