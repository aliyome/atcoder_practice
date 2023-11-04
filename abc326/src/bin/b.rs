use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let f = |n: usize| {
        let hyaku = n / 100;
        let ju = (n / 10) % 10;
        let ichi = n % 10;
        hyaku * ju == ichi
    };

    for i in n..=999 {
        if f(i) {
            println!("{}", i);
            return;
        }
    }
}
