use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }

    // 各日について、次に花火が打ち上げられる日までの日数を計算する
    let mut days_until_next_firework = vec![0; n + 1];
    let mut last_firework_day = 0;

    // 花火が打ち上げられる日についてループを回す
    for &firework_day in &a {
        // last_firework_day+1 から firework_day-1 までの各日について、次の花火までの日数を計算
        for day in (last_firework_day + 1)..firework_day {
            days_until_next_firework[day] = firework_day - day;
        }
        last_firework_day = firework_day;
    }

    // 結果を出力する
    for day in 1..=n {
        println!("{}", days_until_next_firework[day]);
    }
}
