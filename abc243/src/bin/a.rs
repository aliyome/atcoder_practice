use proconio::input;

fn main() {
    input! {
        mut v: isize,
        a: isize,
        b: isize,
        c: isize,
    };

    while true {
        v -= a;
        if v < 0 {
            println!("F");
            return;
        }

        v -= b;
        if v < 0 {
            println!("M");
            return;
        }

        v -= c;
        if v < 0 {
            println!("T");
            return;
        }
    }
}
