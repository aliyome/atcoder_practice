use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [[isize; n]; n],
    };

    let mut a = vec![0; n];
    let mut b = vec![0; n];

    // a を決める
    let mut min = std::isize::MAX;
    for i in 0..n {
        a[i] = c[i][0];
        min = min.min(c[i][0]);
    }
    for i in 0..n {
        a[i] -= min;
    }

    // b を決める
    for j in 0..n {
        b[j] = c[0][j] - a[0];
    }

    // a と b が条件を満たすかチェック
    for i in 0..n {
        for j in 0..n {
            if a[i] + b[j] != c[i][j] {
                println!("No");
                return;
            }
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
