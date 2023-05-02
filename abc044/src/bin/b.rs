use proconio::{input, marker::Chars};

fn main() {
    input! {
        w: Chars,
    };

    let mut m = std::collections::HashMap::new();
    for s in w {
        *m.entry(s).or_insert(0) += 1;
    }

    for (_, v) in m {
        if v % 2 != 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
