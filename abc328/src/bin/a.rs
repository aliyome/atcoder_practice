use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        scores: [i32; n],
    }

    let mut total_score = 0;
    for score in scores {
        if score <= x {
            total_score += score;
        }
    }

    println!("{}", total_score);
}
