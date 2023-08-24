use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut even = 0;
    let mut odd = 0;

    for &a in &a {
        if a % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    let odd_rest = odd % 2;
    even += odd / 2;

    while even > 1 {
        even = (even % 2) + (even / 2);
    }

    if (odd_rest == 1 && even == 0) || (odd_rest == 0 && even == 1) {
        println!("YES");
    } else {
        println!("NO");
    }
}
