use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let result = n.to_string().repeat(n as usize);

    println!("{}", result);
}
