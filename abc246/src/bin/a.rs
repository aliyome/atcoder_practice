use proconio::input;

fn main() {
    input! {
        (x1, y1): (isize, isize),
        (x2, y2): (isize, isize),
        (x3, y3): (isize, isize),
    };

    let mut set_x = std::collections::HashMap::new();
    let mut set_y = std::collections::HashMap::new();

    *set_x.entry(x1).or_insert(0) += 1;
    *set_x.entry(x2).or_insert(0) += 1;
    *set_x.entry(x3).or_insert(0) += 1;
    *set_y.entry(y1).or_insert(0) += 1;
    *set_y.entry(y2).or_insert(0) += 1;
    *set_y.entry(y3).or_insert(0) += 1;

    let mut x = 0;
    for (k, v) in set_x {
        if v == 1 {
            x = k;
            break;
        }
    }

    let mut y = 0;
    for (k, v) in set_y {
        if v == 1 {
            y = k;
            break;
        }
    }

    println!("{} {}", x, y);
}
