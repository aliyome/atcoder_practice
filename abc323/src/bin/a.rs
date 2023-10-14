use proconio::input;

fn main() {
    input! {
        s: String
    }

    if s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .filter(|(i, _)| (*i + 1) != 0 && (*i + 1) % 2 == 0)
        .all(|(_, c)| c == 0)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
