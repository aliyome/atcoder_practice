use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [[usize; n]; n],
    };

    if n == 1 {
        println!("Yes");
        println!("0");
        println!("{}", c[0][0]);
        return;
    }

    // 行ごとの列の差分を求める
    let mut diff_row = vec![vec![0; n]; n];
    for i in 0..n {
        let mut min = std::usize::MAX;
        for j in 0..n {
            min = min.min(c[i][j]);
        }
        for j in 0..n {
            diff_row[i][j] = c[i][j] - min;
        }
    }

    // 列ごとの行の差分を求める
    let mut diff_col = vec![vec![0; n]; n];
    for j in 0..n {
        let mut min = std::usize::MAX;
        for i in 0..n {
            min = min.min(c[i][j]);
        }
        for i in 0..n {
            diff_col[j][i] = c[i][j] - min;
        }
    }

    // 判定
    let mut ok = true;
    for i in 1..n {
        if diff_col[i] != diff_col[0] {
            ok = false;
            break;
        }
        if diff_row[i] != diff_row[0] {
            ok = false;
            break;
        }
    }

    if !ok {
        println!("No");
        return;
    }

    let mut a = diff_col[0].clone();
    let mut b = diff_row[0].clone();

    let c = a[0] as isize - b[0] as isize;
    if c < 0 {
        for i in 0..n {
            a[i] += c.abs() as usize;
        }
    } else {
        for i in 0..n {
            b[i] += c.abs() as usize;
        }
    }

    println!("Yes");
    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
