fn main() {
    let input = read_vec::<i64>();
    let num = input[0];
    let total_price = input[1];

    let result = check_otoshidama(num, total_price);

    println!("{} {} {}", result.0, result.1, result.2);
}

fn check_otoshidama(bill_num: i64, total_price: i64) -> (i64, i64, i64) {
    for i in 0..bill_num + 1 {
        for j in 0..bill_num +1 {
            if i + j > bill_num {
                continue;
            }
            let k = bill_num - i - j;
            if total_price == (i * 10_000) + (j * 5_000) + (k * 1_000) {
                return (i, j, k);
            }
        }
    }
    return (-1, -1, -1);
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
