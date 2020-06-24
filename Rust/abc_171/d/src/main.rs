use std::collections::HashMap;

fn main() {
    let n: i64 = read();
    let nums: Vec<i64> = read_vec();
    let q: i64 = read();
    let operates: Vec<Vec<i64>> = read_vec2(q);

    let mut num_dict = create_dict(&nums);
    let mut num_sum: i64 = num_dict.iter().map(|(key, value)| key * value).sum();

    for operate in operates.iter() {
        let b = operate[0];
        let c = operate[1];

        if let Some(b_count) = num_dict.remove(&b) {
            let c_count = match num_dict.get(&c) {
                Some(i) => i + b_count,
                None => b_count,
            };
            num_dict.insert(c, c_count);

            num_sum = num_sum + (b * -b_count) + (c * b_count);
        }
        
        println!("{}", num_sum);
    }
}

fn create_dict(nums: &Vec<i64>) -> HashMap<i64, i64> {
    let mut d: HashMap<i64, i64> = HashMap::new();
    for num in nums.iter() {
        let count: i64 = match d.get(num) {
            Some(i) => *i + 1,
            None => 1,
        };
        d.insert(*num, count);
    }
    return d;
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

fn read_vec2<T: std::str::FromStr>(n: i64) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
