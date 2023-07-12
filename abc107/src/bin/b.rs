use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    };

    let mut skip_i = vec![];
    let mut skip_j = vec![];
    for i in 0..h {
        let mut skip = true;
        for j in 0..w {
            if a[i][j] == '#' {
                skip = false;
                break;
            }
        }
        if skip {
            skip_i.push(i);
        }
    }
    for j in 0..w {
        let mut skip = true;
        for i in 0..h {
            if a[i][j] == '#' {
                skip = false;
                break;
            }
        }
        if skip {
            skip_j.push(j);
        }
    }
    for i in 0..h {
        if skip_i.contains(&i) {
            continue;
        }
        for j in 0..w {
            if skip_j.contains(&j) {
                continue;
            }
            print!("{}", a[i][j]);
        }
        println!();
    }
}
