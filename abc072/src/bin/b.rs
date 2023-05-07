use proconio::input;

fn main() {
    input! {
        s: String,
    };

    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            print!("{}", c);
        }
    }

    println!("");
}
