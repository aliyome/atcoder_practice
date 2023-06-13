use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
    };

    let mut layers = vec![1; n + 1];
    let mut p = vec![1; n + 1];
    for i in 1..=n {
        layers[i] = layers[i - 1] * 2 + 3;
        p[i] = p[i - 1] * 2 + 1;
    }
    let ans = f(n, x, &layers, &p);
    println!("{}", ans);
}

fn f(n: usize, x: isize, layers: &Vec<isize>, p: &Vec<isize>) -> isize {
    if n == 0 {
        return if x <= 0 { 0 } else { 1 };
    } else if x <= 1 + layers[n - 1] {
        return f(n - 1, x - 1, layers, p);
    } else {
        let left = x - 2 - layers[n - 1];
        return p[n - 1] + 1 + f(n - 1, left, layers, p);
    }
}

// fn main() {
//     input! {
//         n: usize,
//         x: usize,
//     };

//     // 愚直に実装 -> TLE
//     let burger: Vec<char> = make(n, &mut vec![vec![]; 51]);
//     let eaten = &burger[burger.len() - x..];
//     let ans = eaten.iter().filter(|&&c| c == 'P').count();

//     println!("{}", ans);
// }

// fn make(n: usize, mut memo: &mut Vec<Vec<char>>) -> Vec<char> {
//     if memo[n].len() > 0 {
//         return memo[n].clone();
//     }

//     if n == 0 {
//         let b = vec!['P'];
//         memo[n] = b;
//         return memo[n].clone();
//     }

//     let mut ret = vec![];
//     ret.extend(vec!['B']);
//     ret.extend(make(n - 1, &mut memo));
//     ret.extend(vec!['P']);
//     ret.extend(make(n - 1, &mut memo));
//     ret.extend(vec!['B']);
//     memo[n] = ret;

//     return memo[n].clone();
// }
