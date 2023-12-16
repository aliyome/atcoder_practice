use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i32,
        k: i32,
        products: [(i32, i32); n], // 各商品の価格と数量
    }

    // 合計金額を計算する
    let mut total_price = 0;
    for (price, quantity) in products {
        total_price += price * quantity;
    }

    // 配送料を計算する
    let shipping_fee = if total_price >= s { 0 } else { k };

    // 支払うべき合計金額を出力する
    println!("{}", total_price + shipping_fee);
}
