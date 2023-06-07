#[derive(Copy, Clone)]
pub struct TTT {
    pub board: [[i8; 3]; 3],
    pub player: i8,
}

impl TTT {
    pub fn new() -> TTT {
        return TTT { board: [[0i8; 3]; 3], player: 1 }
    }

    pub fn moves(&self) -> Vec<(usize, usize)> {
        (0..3).flat_map(|x| (0..3)
            .map(move |y| (x, y)))
            .collect()
    }

    pub fn p_moves(&self) -> Vec<(usize, usize)> {
        self.moves()
            .into_iter()
            .filter(|(x, y)| self.board[*x][*y] == 0)
            .collect()
    }

    pub fn apply_move(&mut self, m: (usize, usize)) -> &Self {
        self.board[m.0][m.1] = self.player;
        self.player *= -1;
        self
    }

    pub fn best_move(&self, depth: u32) -> (usize, usize) {
        self.p_moves()
            .into_iter()
            .max_by(|x, y| {
                (self.clone()
                    .apply_move(*x)
                    .deep_evaluate(depth) * self.player)
                    .cmp(&(self.clone()
                         .apply_move(*y)
                         .deep_evaluate(depth) * self.player))
            }).unwrap()
    }

    pub fn deep_evaluate(&self, depth: u32) -> i8 {
        if depth == 0 || self.evaluate() != 0 {
            self.evaluate() * (depth + 1) as i8
        } else if self.board_full() {
            0
        } else {
            self.p_moves()
                .into_iter()
                .map(|m| {
                    self.clone()
                        .apply_move(m)
                        .deep_evaluate(depth - 1)
                })
                .fold(-self.player, |x, y| {
                    if x * self.player > y * self.player {
                        x
                    } else {
                        y
                    }
                })
        }
    }

    pub fn board_full(&self) -> bool {
        !self.board
            .iter()
            .flatten()
            .any(|&x| x == 0)
    }

    pub fn evaluate(&self) -> i8 {
        for x in 0..3 {
            for y in 0..3 {
                let c = self.board[x][y];
                if c == 0 {
                    continue;
                }
                {   // horizontal
                    if (0..3).map(|x| self.board[x][y])
                        .all(|x| x == c) {
                            return c
                        }
                }
                {   // vertical
                    if (0..3).map(|y| self.board[x][y])
                        .all(|x| x == c) {
                            return c
                        }
                }
                if x == y { // diagonal 1
                    if (0..3).map(|x| self.board[x][x])
                        .all(|x| x == c) {
                            return c
                        }
                }
                if 2 - x == y { // diagonal 2
                    if (0..3).map(|x| self.board[2 - x][x])
                        .all(|x| x == c) {
                            return c
                        }
                }
            }
        }
        0
    }
}

