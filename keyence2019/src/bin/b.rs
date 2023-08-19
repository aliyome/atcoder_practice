use proconio::input;

fn main() {
    input! {
        s: String
    }
    let target = "keyence";
    let n = s.len();
    let m = target.len();

    for i in 0..=m {
        let pre = &target[0..i];
        let suf = &target[i..m];
        if s.starts_with(pre) && s[(n - m + i)..].ends_with(suf) {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
