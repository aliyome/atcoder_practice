use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };

    if x[0] == x[1] && x[1] == x[2] && x[2] == x[3] {
        println!("Weak");
        return;
    }

    let mut is_weak = true;
    for i in 1..x.len() {
        if x[i - 1].to_digit(10).unwrap() + 1 == x[i].to_digit(10).unwrap()
            || (x[i - 1] == '9' && x[i] == '0')
        {
            continue;
        }
        is_weak = false;
    }
    if is_weak {
        println!("Weak");
        return;
    }
    println!("Strong");
}
