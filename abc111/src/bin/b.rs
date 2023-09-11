use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let a = [111, 222, 333, 444, 555, 666, 777, 888, 999, 1111];

    println!("{}", a[a.partition_point(|&x| x < n)]);
}
