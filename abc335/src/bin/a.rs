use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut chars = s.chars().collect::<Vec<char>>();
    *chars.last_mut().unwrap() = '4';
    let result: String = chars.iter().collect();

    println!("{}", result);
}
