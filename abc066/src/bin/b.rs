use proconio::input;

fn main() {
    input! {
        s: String,
    };

    for i in 1..s.len() {
        if (s.len() - i) % 2 != 0 {
            continue;
        }
        let s = s[0..s.len() - i].to_string();
        let center = s.len() / 2;
        if s[0..center] == s[center..s.len()] {
            println!("{}", s.len());
            return;
        }
    }
    println!("0");
}
