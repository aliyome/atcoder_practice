use proconio::{input, marker::Chars};

fn main() {
    input! {
        ymd: Chars,
    };

    let yyyy: usize = ymd[0..4].iter().collect::<String>().parse().unwrap();
    let mm: usize = ymd[5..7].iter().collect::<String>().parse().unwrap();
    let dd: usize = ymd[8..10].iter().collect::<String>().parse().unwrap();

    // 当月
    for d in dd..=31 {
        if !is_valid_date(yyyy, mm, d) {
            break;
        }
        if yyyy % (mm * d) == 0 {
            println!("{:04}/{:02}/{:02}", yyyy, mm, d);
            return;
        }
    }

    // 当年翌月以降
    for m in mm + 1..=12 {
        for d in 1..=31 {
            if !is_valid_date(yyyy, m, d) {
                break;
            }
            if yyyy % (m * d) == 0 {
                println!("{:04}/{:02}/{:02}", yyyy, m, d);
                return;
            }
        }
    }

    // 翌年以降
    for y in yyyy + 1..=3000 {
        for m in 1..=12 {
            for d in 1..=31 {
                if !is_valid_date(y, m, d) {
                    break;
                }
                if y % (m * d) == 0 {
                    println!("{:04}/{:02}/{:02}", y, m, d);
                    return;
                }
            }
        }
    }
}

fn is_uruu(y: usize) -> bool {
    y % 4 == 0 && y % 100 != 0 || y % 400 == 0
}

fn is_valid_date(y: usize, m: usize, d: usize) -> bool {
    if [2, 4, 6, 9, 11].contains(&m) && d > 30 {
        return false;
    }
    if m == 2 {
        if is_uruu(y) {
            return d <= 29;
        } else {
            return d <= 28;
        }
    }
    d <= 31
}
