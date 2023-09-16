use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        m: usize,
        s1: String,
        s2: String,
        s3: String,
    }

    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    let s3 = s3.chars().collect::<Vec<char>>();

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    let mut map3 = HashMap::new();

    for (i, &ch) in s1.iter().enumerate() {
        map1.entry(ch).or_insert(vec![]).push(i);
    }
    for (i, &ch) in s2.iter().enumerate() {
        map2.entry(ch).or_insert(vec![]).push(i);
    }
    for (i, &ch) in s3.iter().enumerate() {
        map3.entry(ch).or_insert(vec![]).push(i);
    }

    let mut answer = std::usize::MAX;
    for (&key, val1) in &map1 {
        if let Some(val2) = map2.get(&key) {
            if let Some(val3) = map3.get(&key) {
                for &i in val1 {
                    for &j in val2 {
                        for &k in val3 {
                            let total_time = i + j + k;
                            answer = std::cmp::min(answer, total_time);
                        }
                    }
                }
            }
        }
    }

    if answer == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", answer);
    }
}
