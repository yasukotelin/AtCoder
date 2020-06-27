fn main() {
    let s: Vec<char> = read::<String>().chars().collect();
    let t: Vec<char> = read::<String>().chars().collect();

    let mut count = 0;
    for i in 0..s.len() {
        if s[i] != t[i] {
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
