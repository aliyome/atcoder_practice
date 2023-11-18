use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let result = s
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);
}
