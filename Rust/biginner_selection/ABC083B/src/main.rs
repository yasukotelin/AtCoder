fn main() {
    let nums = read_vec::<i64>();
    let n = nums[0];
    let a = nums[1];
    let b = nums[2];

    let mut list: Vec<i64> = Vec::new();
    for v in 0..n + 1 {
        let added = add(&v);
        if a <= added && added <= b {
            list.push(v)
        }
    }

    println!("{}", list.iter().sum::<i64>())
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

fn add(num: &i64) -> i64 {
    num.to_string()
        .chars()
        .map(|v| v.to_string().parse::<i64>().unwrap())
        .sum::<i64>()
}
