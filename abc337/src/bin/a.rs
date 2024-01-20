use proconio::input;

fn main() {
    input! {
        n: usize,
        scores: [(i32, i32); n],
    }

    let (mut takahashi_total, mut aoki_total) = (0, 0);
    for (takahashi_score, aoki_score) in scores {
        takahashi_total += takahashi_score;
        aoki_total += aoki_score;
    }

    if takahashi_total > aoki_total {
        println!("Takahashi");
    } else if aoki_total > takahashi_total {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
