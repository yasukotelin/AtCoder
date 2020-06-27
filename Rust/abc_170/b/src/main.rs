fn main() {
    let inputs: Vec<i32> = read_vec();
    let x = inputs[0];
    let y = inputs[1];

    for i in 0..x + 1 {
        let crane = i;
        let turtle = x - i;

        let legs = (crane * 2) + (turtle * 4);
        if legs == y {
            println!("Yes");
            return;
        }
    }

    println!("No");
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
