fn main() {
    let items = ["dream", "dreamer", "erase", "eraser"];
    let s = read::<String>();

    // 逆転させる
    let items = items
        .iter()
        .map(|v| v.chars().rev().collect::<String>())
        .collect::<Vec<String>>();
    let target: String = s.chars().rev().collect();

    let mut index = 0;
    let mut can_make = true;
    while index < target.len() {
        let mut can = false;
        for item in &items {
            let start = index;
            let end = index + item.len();
            if target.len() < end {
                continue;
            }
            let v = &target[start..end];
            if v == item {
                index += item.len();
                can = true;
                break;
            }
        }
        // itemと一致しなかった場合
        if !can {
            can_make = false;
            break;
        }
    }

    println!("{}", if can_make { "YES" } else { "NO" });
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
