fn main() {
    let n: u64 = read();
    let nums: Vec<u64> = read_vec();
    let text: String = read();

    let mut count = n;
    for i in nums {
        count += i;
    }

    println!("{} {}", count, text);
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
