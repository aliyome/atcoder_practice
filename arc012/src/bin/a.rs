use proconio::input;

fn main() {
    input! {
        day: String,
    };

    let ans = match day.as_str() {
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        "Friday" => 1,
        _ => 0,
    };

    println!("{}", ans);
}
