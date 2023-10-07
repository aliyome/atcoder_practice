use proconio::input;

fn main() {
    input! {
        s: String
    }

    for i in (2..=16).step_by(2) {
        if s.chars().nth(i - 1).unwrap() != '0' {
            // 0でない場合、"No"を出力して終了
            println!("No");
            return;
        }
    }

    println!("Yes");
}
