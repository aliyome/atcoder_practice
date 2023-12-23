use proconio::input;

fn main() {
    input! {
        b: i32, // Bat cost
        g: i32, // Glove cost
    }

    if b > g {
        println!("Bat");
    } else {
        println!("Glove");
    }
}
