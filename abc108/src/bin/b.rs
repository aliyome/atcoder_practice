use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    };

    // xy1 -> xy2 のベクトル
    let vx = x2 - x1;
    let vy = y2 - y1;

    // 90度回転させる
    // | X | = | cos(90) = 0, -sin(90) = -1 | | x |
    // | Y |   | sin(90) = 1, cos(90) = 0   | | y |
    // X = -y
    // Y = x
    let dx = -vy;
    let dy = vx;
    let vx = dx;
    let vy = dy;

    let x3 = x2 + dx;
    let y3 = y2 + dy;

    // 90度回転させる
    let dx = -vy;
    let dy = vx;
    let vx = dx;
    let vy = dy;

    let x4 = x3 + dx;
    let y4 = y3 + dy;

    println!("{} {} {} {}", x3, y3, x4, y4);
}
