fn main() {
    let n: i32 = read();
    let mut inputs: Vec<f64> = Vec::new();

    for _ in 0..n {
        let a: f64 = read();
        inputs.push(a);
    }

    let mut count = 0;

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let a = inputs[i as usize] * inputs[j as usize];
            
            // aの整数判定
            let is_int = a % 1.0 == 0.0;

            if is_int {
                count += 1;
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
