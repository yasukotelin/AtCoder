fn main() {
    let n = read::<u32>();
    let numbers = read_vec::<u32>();
    for num in numbers.iter() {
        if num % 2 != 0 {
            continue;
        }

        if num % 3 != 0 && num % 5 != 0 {
            println!("DENIED");
            return;
        }
    }

    println!("APPROVED");
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
