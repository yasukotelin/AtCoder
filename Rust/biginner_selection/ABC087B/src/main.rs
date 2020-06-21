fn main() {
    let coin_500 = read::<u64>();
    let coin_100 = read::<u64>();
    let coin_50 = read::<u64>();
    let price = read::<u64>();

    let mut count = 0;
    for i in 0..coin_500+1 {
        for j in 0..coin_100+1 {
            for k in 0..coin_50+1 {
                if 500*i + 100*j + 50*k == price {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
