use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            mut p: [usize; n]
        };
        p.insert(0, 0);

        let mut ans = 0;
        for i in 1..n {
            let mut ok = true;
            for j in i + 1..=n {
                if p[i] > p[j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
            }
        }

        println!("{}", ans + 1);
    }
}

//  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
// 13  2  7  1  5  9  3  4 12 10 15  6  8 14 20 16 19 18 11 17
