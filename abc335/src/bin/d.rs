use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // グリッドの初期化
    let mut grid = vec![vec![0; n]; n];
    let mut x = 0usize;
    let mut y = 0usize;
    let mut dx = 1i32;
    let mut dy = 0i32;
    let mut part = 1;

    while part < n * n {
        // Takahashiを中心に配置
        if 2 * x + 1 == n && 2 * y + 1 == n {
            grid[x][y] = 0;
            break;
        }

        // ドラゴンの部分を配置
        grid[x][y] = part;
        part += 1;

        // 次のセルに移動
        let nx = (x as i32 + dx) as usize;
        let ny = (y as i32 + dy) as usize;

        // 壁または既に配置されたセルに当たったら方向転換
        if nx >= n || ny >= n || grid[nx][ny] != 0 {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }

        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
    }

    // 結果の出力
    for row in grid.iter() {
        for &cell in row.iter() {
            if cell == 0 {
                print!("T ");
            } else {
                print!("{} ", cell);
            }
        }
        println!();
    }
}
