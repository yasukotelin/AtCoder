fn main() {
    let word: String = read();
    if word.chars().last().unwrap() == 's' {
        println!("{}es", word);
    } else {
        println!("{}s", word);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
