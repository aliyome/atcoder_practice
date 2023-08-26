use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: String,
    };

    println!("{}", s.replace("2014", "2015"));
}
