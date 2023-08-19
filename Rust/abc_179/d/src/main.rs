use std::collections::HashSet;

fn main() {
    let input: Vec<u32> = read_vec();
    let n = input[0];
    let k = input[1];

    let lr_list:Vec<Vec<u32>> = read_vec2(k);

    let union: HashSet<u32> = lr_list.into_iter().flatten().collect();

    let mut pos = 1;
    for d in union.iter() {
        pos = pos + d;
        if pos < n {
            continue;
        } else if pos == n {
            
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
