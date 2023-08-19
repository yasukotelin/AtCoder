fn main() {
    let n: u32 = read();
    let data_list: Vec<Vec<u32>> = read_vec2(n);

    let mut counter = 0;

    for data in data_list.iter() {
        let v1 = data[0];
        let v2 = data[1];
        if v1 == v2 {
            // ゾロ目
            counter += 1;

            if counter == 3 {
                println!("Yes");
                return;
            }
        } else {
            counter = 0;
        }
    }

    println!("No")
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

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
