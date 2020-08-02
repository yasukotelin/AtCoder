fn main() {
    let n: u32 = read();
    let mut yen = 0;
    loop {
        yen += 1000;
        if yen >= n {
            println!("{}", yen - n);
            return;
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
