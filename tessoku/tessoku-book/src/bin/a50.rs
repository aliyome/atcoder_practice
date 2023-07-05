use proconio::input;
use rand::Rng;

const N: usize = 100;

fn main() {
    input! {
      a: [[isize; N]; N]
    }

    // 低いほうが良い
    let calc_score = |grid: &Vec<Vec<isize>>| -> isize {
        let mut score = 0;
        for j in 0..N {
            for i in 0..N {
                score += (a[j][i] - grid[j][i]).abs();
            }
        }
        score
    };

    // 操作
    let exec_op = |grid: &mut Vec<Vec<isize>>, x: usize, y: usize, h: usize| {
        for j in 0..N {
            for i in 0..N {
                let cost = (h as isize
                    - (x as isize - i as isize).abs()
                    - (y as isize - j as isize).abs())
                .max(0);
                grid[j][i] += cost;
            }
        }
    };

    // 最大1000回 ランダムに試行
    // NxNx1000 -> 10^7
    let mut grid = vec![vec![0isize; N]; N];
    let mut operations = vec![];
    let mut scores = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let x = rng.gen_range(0, N);
        let y = rng.gen_range(0, N);
        let h = rng.gen_range(1, N);
        let op = (x, y, h);
        exec_op(&mut grid, x, y, h);
        operations.push(op);
        let score = calc_score(&grid);
        scores.push(score);
    }

    let mut min_score = std::isize::MAX;
    let mut min_index = 0;
    for i in 0..1000 {
        if scores[i] < min_score {
            min_score = scores[i];
            min_index = i;
        }
    }

    println!("{}", min_index);
    for i in 0..min_index {
        println!(
            "{} {} {}",
            operations[i].0, operations[i].1, operations[i].2
        );
    }
}
