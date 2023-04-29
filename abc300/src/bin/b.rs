use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h]
    };

    for s in 0..h {
        for t in 0..w {
            let mut a_shifted = a.clone();
            for _ in 0..s {
                a_shifted = shift_vertical(a_shifted);
            }
            for _ in 0..t {
                a_shifted = shift_horizontal(a_shifted);
            }
            if a_shifted == b {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

fn shift_vertical(a: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = a.len();
    let mut shifted = a.clone();
    for i in 0..h {
        shifted[i] = a[(i + 1) % h].clone();
    }
    shifted
}

fn shift_horizontal(a: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = a.len();
    let w = a[0].len();
    let mut shifted = a.clone();
    for i in 0..h {
        for j in 0..w {
            shifted[i][j] = a[i][(j + 1) % w];
        }
    }
    shifted
}
