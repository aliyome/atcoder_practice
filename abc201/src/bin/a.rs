use proconio::input;

fn main() {
    input! {
        mut a: [isize; 3],
    };

    a.sort();

    if a[2] - a[1] == a[1] - a[0] {
        println!("Yes");
    } else {
        println!("No");
    }
}
