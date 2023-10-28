use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    // Nから始めて1ずつ増やしながら326-like numberを探す
    for num in n..=919 {
        // 整数を文字列に変換
        let num_str = num.to_string();
        // 各桁の値を取得
        let hundreds_digit = num_str.chars().nth(0).unwrap().to_digit(10).unwrap();
        let tens_digit = num_str.chars().nth(1).unwrap().to_digit(10).unwrap();
        let ones_digit = num_str.chars().nth(2).unwrap().to_digit(10).unwrap();

        // 326-like numberか判断
        if hundreds_digit * tens_digit == ones_digit {
            println!("{}", num);
            return;
        }
    }
}
