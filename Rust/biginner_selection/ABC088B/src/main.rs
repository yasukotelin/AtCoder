struct Player {
    cards: Vec<i64>,
}

impl Player {
    fn new() -> Player {
        Player { cards: Vec::new() }
    }

    fn push(&mut self, card: i64) {
        self.cards.push(card);
    }

    fn get_total(&self) -> i64 {
        self.cards.iter().sum()
    }
}

struct Field {
    cards: Vec<i64>,
}

impl Field {
    fn can_draw(&self) -> bool {
        self.cards.len() > 0
    }

    fn draw(&mut self) -> i64 {
        let first = self.cards.first();
        if first == None {
            return 0;
        }
        let mut max = first.unwrap();
        let mut max_index = 0;
        for (i, v) in self.cards.iter().enumerate() {
            if max < v {
                max = v;
                max_index = i;
            }
        }
        self.cards.remove(max_index)
    }
}

fn main() {
    let n = read::<i64>();
    let cards = read_vec::<i64>();

    let mut field = Field { cards: cards };
    let mut alice = Player::new();
    let mut bob = Player::new();

    let mut turn = 1; // 1がAlice, -1がBob
    while field.can_draw() {
        let card = field.draw();
        
        let player = if turn == 1 { &mut alice } else { &mut bob };
        player.push(card);

        turn *= -1;
    }

    let diff = alice.get_total() - bob.get_total();
    println!("{}", diff);
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
