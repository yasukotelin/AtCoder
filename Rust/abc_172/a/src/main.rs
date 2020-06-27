fn main() {
    let a: i32 = read();
    println!("{}", a + (a * a) + (a * a * a))
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
