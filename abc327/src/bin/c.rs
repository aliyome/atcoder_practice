use proconio::input;

fn main() {
    input! {
        a: [[u32; 9]; 9], // 9x9 grid input
    }

    if is_valid_sudoku(&a) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_valid_sudoku(grid: &Vec<Vec<u32>>) -> bool {
    // 行のチェック
    for row in grid {
        if !check_unique(row) {
            return false;
        }
    }

    // 列のチェック
    for col in 0..9 {
        let mut column = vec![0; 9];
        for row in 0..9 {
            column[row] = grid[row][col];
        }
        if !check_unique(&column) {
            return false;
        }
    }

    // 3x3 サブグリッドのチェック
    for block_row in 0..3 {
        for block_col in 0..3 {
            let mut block = vec![0; 9];
            for i in 0..3 {
                for j in 0..3 {
                    block[i * 3 + j] = grid[block_row * 3 + i][block_col * 3 + j];
                }
            }
            if !check_unique(&block) {
                return false;
            }
        }
    }

    true
}

// 数字が1から9まで正確に1回ずつ含まれているかをチェックするヘルパー関数
fn check_unique(numbers: &Vec<u32>) -> bool {
    let mut seen = [false; 9];
    for &num in numbers {
        if num < 1 || num > 9 || seen[(num - 1) as usize] {
            return false;
        }
        seen[(num - 1) as usize] = true;
    }
    true
}
