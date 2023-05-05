use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n]
    };

    for i in 0..n {
        let ai = &st[i];
        let mut ok_1 = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            if ai.0 == st[j].0 || ai.0 == st[j].1 {
                ok_1 = false;
            }
        }
        let mut ok_2 = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            if ai.1 == st[j].0 || ai.1 == st[j].1 {
                ok_2 = false;
            }
        }
        if !ok_1 && !ok_2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
