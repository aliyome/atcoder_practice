use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    for a in a {
        if a % 2 != 0 {
            continue;
        }

        if a % 3 != 0 && a % 5 != 0 {
            println!("DENIED");
            return;
        }
    }

    println!("APPROVED");
}
