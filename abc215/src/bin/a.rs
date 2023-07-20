use proconio::input;

fn main() {
    input! {
        s: String,
    };

    if s == "Hello,World!" {
        println!("AC");
    } else {
        println!("WA");
    }
}
