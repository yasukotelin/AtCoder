fn main() {
    let n = read::<i64>();
    let mut nums = read_vec::<i64>();

    let mut count = 0;
    while is_even(&nums) {
        count += 1;
        nums = div2(&nums);
    }

    println!("{}", count);
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

fn is_even(nums: &[i64]) -> bool {
    for n in nums {
        if n % 2 != 0 {
            return false
        }
    }
    return true
}

fn div2(nums: &Vec<i64>) -> Vec<i64> {
    nums.iter().map(|v| v / 2).collect()
}