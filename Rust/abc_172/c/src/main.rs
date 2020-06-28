fn main() {
    let inputs: Vec<i64> = read_vec();
    let n_books = inputs[0];
    let m_books = inputs[1];
    let total_limit_time = inputs[2];
    let n_read_times: Vec<i64> = read_vec();
    let m_read_times: Vec<i64> = read_vec();

    // 事前に足し合わせる
    let n_read_times = add_up(&n_read_times);
    let m_read_times = add_up(&m_read_times);

    let mut bests: Vec<usize> = Vec::new();
    for (n_book_count, n_time) in n_read_times.iter().enumerate() {
        if n_time > &total_limit_time {
            break;
        }
        let diff = total_limit_time - n_time;

        let m_book_count = binary_search(&m_read_times, diff);
        bests.push(n_book_count + m_book_count);
    }
    bests.sort();
    let max = bests.last().unwrap();

    println!("{}", max);
}

fn add_up(read_times: &Vec<i64>) -> Vec<i64> {
    let mut vec: Vec<i64> = Vec::new();
    let mut total = 0;
    vec.push(total);
    for time in read_times.iter() {
        total += time;
        vec.push(total);
    }

    return vec;
}

fn binary_search(vec: &Vec<i64>, target: i64) -> usize {
    let mut start = 0;
    let mut end = vec.len();
    let mut center = vec.len() / 2;
    loop {
        if start == center {
            return center;
        }
        if vec[center] == target {
            return center;
        } else if vec[center] > target {
            // Left
            end = center;
        } else {
            // right
            start = center;
        }

        center = start + ((end - start) / 2);
    }
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
