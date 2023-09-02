use proconio::input;

fn main() {
    input! {
        n: usize,
        sheets: [(usize, usize, usize, usize); n]
    }

    let mut area = 0;
    for y in 0..=100 {
        let mut x_count = vec![0; 101];
        for &(a, b, c, d) in &sheets {
            if c <= y && y < d {
                x_count[a] += 1;
                x_count[b] -= 1;
            }
        }
        let mut cnt = 0;
        for x in 0..=100 {
            cnt += x_count[x];
            if cnt > 0 {
                area += 1;
            }
        }
    }
    println!("{}", area);
}
