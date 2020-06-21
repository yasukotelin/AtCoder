struct PositionInfo {
    sec: i64,
    x: i64,
    y: i64,
}

fn main() {
    let n = read::<u32>();
    let lines = read_vec2::<i64>(n);

    if judge_possible_plan(&lines) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn judge_possible_plan(lines: &Vec<Vec<i64>>) -> bool {
    // 原点で初期化
    let mut pos = PositionInfo { sec: 0, x: 0, y: 0 };

    for line in lines.iter() {
        let next = PositionInfo {
            sec: line[0],
            x: line[1],
            y: line[2],
        };
        // 前回地点から今回地点の移動量
        let move_amount = (next.x - pos.x).abs() + (next.y - pos.y).abs();
        let sec_diff = next.sec - pos.sec;

        // 移動量が秒差分よりも大きい場合は移動不可
        if move_amount > sec_diff {
            return false;
        }

        if sec_diff % 2 == 0 && move_amount % 2 != 0 {
            // 偶数秒の時に奇数移動量はありえない
            return false;
        }
        if sec_diff % 2 != 0 && move_amount % 2 == 0 {
            // 奇数秒の時に偶数移動量はありえない
            return false;
        }

        pos = next;
    }
    return true;
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

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
