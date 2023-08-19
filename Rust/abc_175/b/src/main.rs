fn main() {
    let n: i32 = read();
    let l_list: Vec<i32> = read_vec();

    let mut count = 0;

    // 三角形の条件
    // |a−b| < c < a + b
    for i in 0..(n-2) {
        let a = l_list[i as usize];
        for j in i..(n-1) {
            let b = l_list[j as usize];
            if a == b {
                continue;
            }

            for k in j..n {
                let c = l_list[k as usize];
                if a == c || b == c {
                    continue;
                }

                if (a - b).abs() < c && c < (a + b) {
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

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
