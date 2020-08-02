fn main() {
    let k: u32 = read();

    let prime_number = check_prime_number(k);

    if !prime_number {
        println!("-1");
        return;
    }

    println!("{}", prime_number);
}

fn check_prime_number(num: u32) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
