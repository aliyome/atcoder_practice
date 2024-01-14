use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    println!("{}", find_nth_good_number(n));
}

// 「良い整数」を見つけるための関数
fn find_nth_good_number(n: u64) -> u64 {
    let mut good_numbers = vec![];
    generate_good_numbers(0, n as usize, &mut good_numbers);
    good_numbers.sort_unstable();
    good_numbers[n as usize - 1]
}

// 「良い整数」を生成するための再帰関数
fn generate_good_numbers(current: u64, limit: usize, good_numbers: &mut Vec<u64>) {
    if good_numbers.len() >= limit {
        return;
    }
    if is_good_number(current) {
        good_numbers.push(current);
    }
    for &digit in [0, 2, 4, 6, 8].iter() {
        let next_number = current * 10 + digit;
        if next_number != 0 {
            generate_good_numbers(next_number, limit, good_numbers);
        }
    }
}

// 数が「良い整数」かどうかを判断する関数
fn is_good_number(num: u64) -> bool {
    num == 0
        || num
            .to_string()
            .chars()
            .all(|c| matches!(c, '0' | '2' | '4' | '6' | '8'))
}
