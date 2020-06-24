fn main() {
    let c = read::<char>();

    if 'A' <= c && c <= 'Z' {
        println!("A");
    } else {
        println!("a");
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
