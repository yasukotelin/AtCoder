fn main() {
    let n: i32 = read();
    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;
    for _ in 0..n {
        let s: String = read();
        match s.as_ref() {
            "AC" => ac += 1,
            "WA" => wa += 1,
            "TLE" => tle += 1,
            _ => re += 1,
        }
    }

    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
