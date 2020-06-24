fn main() {
    let read: Vec<u32> = read_vec();
    let _ = read[0];
    let num = read[1];
    let mut prices: Vec<u32> = read_vec();

    prices.sort();
    let mut price = 0;
    for i in 0..num {
        price += prices[i as usize];
    }

    println!("{}", price);
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

