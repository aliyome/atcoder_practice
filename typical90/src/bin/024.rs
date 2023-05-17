use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
        b: [usize; n],
    }

    let mut diff = 0;
    for i in 0..n {
        diff += (a[i] as isize - b[i] as isize).abs();
    }
    if k < diff as usize {
        println!("No");
        return;
    }
    if (diff - k as isize) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
