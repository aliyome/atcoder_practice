use proconio::input;

fn main() {
    input! {
        n: usize,
        bases: [(u32, u32); n]
    }

    // UTCの0時から23時までの各時間を会議の開始時間として考える
    let mut max_participation = 0;
    for start_hour in 0..24 {
        let mut participation = 0;
        for &(employees, local_offset) in &bases {
            let local_start = (start_hour + local_offset) % 24;
            // 各拠点の従業員が参加できるかどうかをチェック
            if local_start >= 9 && local_start <= 17 {
                participation += employees;
            }
        }
        max_participation = max_participation.max(participation);
    }

    println!("{}", max_participation);
}
