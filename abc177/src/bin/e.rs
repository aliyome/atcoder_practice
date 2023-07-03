use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let A_MAX: usize = 10usize.pow(6);

    let mut count = vec![0; A_MAX + 1];
    for i in 0..n {
        count[a[i]] += 1;
    }

    let mut is_pairwise_coprime = true;
    for i in 2..=A_MAX {
        let mut cnt = 0;
        for j in (i..=A_MAX).step_by(i) {
            cnt += count[j];
        }
        if cnt > 1 {
            is_pairwise_coprime = false;
            break;
        }
    }
    if is_pairwise_coprime {
        println!("pairwise coprime");
        return;
    }

    let mut g = 0;
    for &a in &a {
        g = gcd(g, a);
    }
    if g == 1 {
        println!("setwise coprime");
    } else {
        println!("not coprime");
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
