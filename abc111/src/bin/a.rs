use proconio::input;

fn main() {
    input! {
        n: String,
    };

    let n = n
        .chars()
        .map(|c| match c {
            '1' => '9',
            '9' => '1',
            _ => c,
        })
        .collect::<String>();

    println!("{}", n);
}
