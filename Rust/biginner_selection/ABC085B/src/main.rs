fn main() {
    let n = read::<i64>();
    let mut rice_cakes: Vec<i64> = Vec::new();
    for _ in 0..n {
        rice_cakes.push(read());
    }

    rice_cakes.sort();
    rice_cakes.dedup();

    println!("{}", rice_cakes.len());
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
