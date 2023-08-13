use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    };

    let mut set = HashSet::new();
    set.insert("ABC");
    set.insert("ARC");
    set.insert("AGC");
    set.insert("AHC");

    set.remove(s1.as_str());
    set.remove(s2.as_str());
    set.remove(s3.as_str());

    println!("{}", set.iter().next().unwrap());
}
