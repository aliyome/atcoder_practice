use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        py: [(usize, usize); m]
    };

    let mut prefecture = vec![vec![]; n + 1];

    // O(M)
    for (i, &(p, y)) in py.iter().enumerate() {
        prefecture[p].push((i + 1, y));
    }

    // O(N)
    let mut ids = vec![String::new(); m + 1];
    let gen =
        |prefecture_id: usize, x: usize| -> String { format!("{:06}{:06}", prefecture_id, x) };
    for p in 1..=n {
        // year順にソート
        prefecture[p].sort_by(|a, b| a.1.cmp(&b.1));
        // 認識番号を生成
        let mut x = 0;
        for &(city_id, _) in prefecture[p].iter() {
            x += 1;
            ids[city_id] = gen(p, x);
        }
    }

    // O(M)
    for i in 1..=m {
        println!("{}", ids[i]);
    }
}
