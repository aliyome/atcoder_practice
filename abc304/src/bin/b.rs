use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n <= 10usize.pow(3) - 1 {
        println!("{}", n);
    } else if n <= 10usize.pow(4) - 1 {
        let x = n / 10;
        println!("{}", x * 10)
    } else if n <= 10usize.pow(5) - 1 {
        let x = n / 100;
        println!("{}", x * 100)
    } else if n <= 10usize.pow(6) - 1 {
        let x = n / 1000;
        println!("{}", x * 1000)
    } else if n <= 10usize.pow(7) - 1 {
        let x = n / 10000;
        println!("{}", x * 10000)
    } else if n <= 10usize.pow(8) - 1 {
        let x = n / 100000;
        println!("{}", x * 100000)
    } else if n <= 10usize.pow(9) - 1 {
        let x = n / 1000000;
        println!("{}", x * 1000000)
    }
}
