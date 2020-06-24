fn main() {
    let alphas = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    let alphas_len = 26;
    let mut n: usize = read();

    let mut mods = Vec::new();
    while n > 0 {
        // スライスに合わせるためにn-1する
        n -= 1;
        
        let m = n % alphas_len;
        mods.push(m);

        n = n / alphas_len;
    }

    let name = mods
        .iter()
        .rev()
        .map(|v| (alphas[*v]).to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", name);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
