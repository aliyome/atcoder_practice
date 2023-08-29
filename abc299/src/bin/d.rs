use std::io::{stdin, BufReader};

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    };

    let mut ok = 0;
    let mut ng = n;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        println!("? {}", mid);

        input! {
            from &mut source,
            x: usize
        }

        if x == 0 {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("! {}", ok);
}
