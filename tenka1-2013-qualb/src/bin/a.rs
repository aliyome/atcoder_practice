use proconio::input;

fn main() {
    input! {
        mut x: [String; 15],
    };

    x.sort();
    println!("{}", x[6]);
}
