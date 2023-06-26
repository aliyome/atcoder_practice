use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n]
    };

    let mut count = 0;
    for (d1, d2) in d {
        if count >= 3 {
            break;
        }
        if d1 == d2 {
            count += 1;
        } else {
            count = 0;
        }
    }

    if count >= 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
