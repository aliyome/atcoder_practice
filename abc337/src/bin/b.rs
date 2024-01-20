use proconio::input;

fn main() {
    input! {
        s: String,
    }

    fn is_extended_abc_string(s: &str) -> bool {
        let mut phase = 0;

        for c in s.chars() {
            match c {
                'A' if phase <= 1 => phase = 1,
                'B' if phase <= 2 => phase = 2,
                'C' if phase <= 3 => phase = 3,
                _ => return false,
            }
        }

        phase >= 1 // 少なくともAの部分が存在することを確認
    }

    if is_extended_abc_string(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
