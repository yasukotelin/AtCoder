use std::collections::HashMap;

fn main() {
    let inputs: Vec<i32> = read_vec();
    let x = inputs[0];

    let nums: Vec<i32> = read_vec();
    let num_dict = conv_dict(&nums);

    // xからマイナス方向に探索
    let mut min_num = 0;
    // x自身も含めることに注意
    for i in (0..x + 1).rev() {
        if let Some(_) = num_dict.get(&i) {
            continue;
        }
        min_num = i;
        break;
    }

    // xからプラス方向に探索
    let mut max_num = 0;
    let mut i = x + 1;
    loop {
        if let Some(_) = num_dict.get(&i) {
            i += 1;
            continue;
        }
        max_num = i;
        break;
    }

    // 最短を計算
    let result = if (x - min_num).abs() <= (max_num - x).abs() {
        min_num
    } else {
        max_num
    };

    println!("{}", result);
}

fn conv_dict(nums: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for num in nums {
        map.insert(*num, 0);
    }
    return map;
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
