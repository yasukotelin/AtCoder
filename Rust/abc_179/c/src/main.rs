fn main() {
    let n: u32 = read();

    let mut counter = 0;

    for a in 1..n {
        for b in 1..n {
            let a_x_b = a * b;
            if a_x_b >= n {
                break;
            }

            counter += 1;
        }
    }

    println!("{}", counter);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
