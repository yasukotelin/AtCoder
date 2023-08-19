fn main() {
    let s: Vec<char> = read::<String>().chars().collect();

    let mut max_count = 0;
    let mut count = 0;
    for c in s.iter() {
        if *c == 'R' {
            count += 1;
        } else {
            count = 0;
        }

        if max_count < count {
            max_count = count;
        }
    }

    println!("{}", max_count);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
