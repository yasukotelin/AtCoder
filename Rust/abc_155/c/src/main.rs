use std::collections::HashMap;

fn main() {
    let n = read::<u32>();
    let mut words: Vec<String> = Vec::new();
    for _ in 0..n {
        words.push(read());
    }

    let mut dict: HashMap<String, u32> = HashMap::new();
    let mut max = 0;
    for word in words.iter() {
        let d = dict.get(word);
        let new_count = match d {
            Some(i) => i + 1,
            None => 1,
        };
        if new_count > max {
            max = new_count;
        }
        dict.insert(String::from(word), new_count);
    }

    let mut max_words: Vec<_> = dict
        .iter()
        .filter(|&(k, v)| v == &max)
        .map(|(k, v)| k)
        .collect();
    max_words.sort();

    for word in &max_words {
        println!("{}", word);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
