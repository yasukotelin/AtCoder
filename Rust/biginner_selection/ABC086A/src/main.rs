fn main() {
    let nums: Vec<i64> = read_vec();
    let product = nums[0] * nums[1];

    if product % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
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
