use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let mut set = HashSet::new();
    set.insert("ACE");
    set.insert("BDF");
    set.insert("CEG");
    set.insert("DFA");
    set.insert("EGB");
    set.insert("FAC");
    set.insert("GBD");

    if set.contains(&s.as_str()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
