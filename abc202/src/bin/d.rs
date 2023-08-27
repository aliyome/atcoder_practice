use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        k: usize,
    };

    let c = nck(a + b);
    let mut cnt = 0;

    for _ in 0..a + b {
        if a == 0 {
            print!("b");
            continue;
        } else if b == 0 {
            print!("a");
            continue;
        }

        if k <= c[a - 1 + b][a - 1] + cnt {
            a -= 1;
            print!("a");
        } else {
            cnt += c[a - 1 + b][a - 1];
            b -= 1;
            print!("b");
        }
    }
}

fn nck(n: usize) -> Vec<Vec<usize>> {
    let mut c = vec![vec![0; n + 1]; n + 1];
    c[0][0] = 1;
    for i in 1..n {
        c[i][0] = 1;
        for j in 1..n {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    c
}
