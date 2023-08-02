use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    };

    // 1->2のベクトル
    let v = (x2 - x1, y2 - y1);

    // 90度回転
    let rotate = |(x, y)| (-1 * y, x);

    // 2->3のベクトル
    let v3 = rotate(v);
    let xy3 = (x2 + v3.0, y2 + v3.1);

    // 3->4のベクトル
    let v4 = rotate(v3);
    let xy4 = (xy3.0 + v4.0, xy3.1 + v4.1);

    println!("{} {} {} {}", xy3.0, xy3.1, xy4.0, xy4.1);
}
