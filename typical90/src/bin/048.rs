use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        ab: [(usize, usize); n]
    }

    // 1分の価値を取り出す
    let mut list = vec![];
    for &(a, b) in &ab {
        // 1分で解ける部分点
        list.push(b);
        // 部分点を取ったあとに1分で追加できる点
        // MEMO: b > (a-b) なので、必ず b が先に取り出される
        list.push(a - b);
    }

    // 1分の価値が高い順にソートする
    list.sort();
    list.reverse();

    let mut ans = 0;
    let mut i = 0;
    for &p in &list {
        ans += p;
        i += 1;
        if i == k {
            break;
        }
    }

    println!("{}", ans);
}
