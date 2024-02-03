use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let last_substring = s.split('.').last().unwrap();

    println!("{}", last_substring);
}
