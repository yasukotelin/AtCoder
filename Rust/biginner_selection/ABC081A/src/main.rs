fn main() {
    let chars: Vec<char> = read::<String>().chars().collect();

    let mut count = 0;
    for c in chars {
        if c == '1' {
            count += 1;
        }
    }

    println!("{}", count);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}