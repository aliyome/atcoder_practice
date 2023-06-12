use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n]
    };

    // 締め切りが早い順に並び替える
    ab.sort_by(|a, b| (a.1).cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut time = 0;
    for (a, b) in ab {
        time += a;
        if time > b {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
