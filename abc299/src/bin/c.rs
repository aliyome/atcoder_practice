use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let is_all_dango = s.iter().all(|&c| c == '-');
    let is_all_stick = s.iter().all(|&c| c == 'o');
    if is_all_dango || is_all_stick {
        println!("-1");
        return;
    }

    let mut ans = 0;
    for x in s.iter().collect::<String>().split('-') {
        ans = ans.max(x.len());
    }

    println!("{}", ans);
}
