fn main() {
    let inputs: Vec<u32> = read_vec();
    let n = inputs[0];
    let d = inputs[1] as f64;
    let inputs: Vec<Vec<i64>> = read_vec2(n);

    let mut count = 0;
    for input in inputs.iter() {
        let x = input[0];
        let y = input[1];

        if ((x.pow(2) + y.pow(2)) as f64).sqrt() <= d {
            count += 1;
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

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
