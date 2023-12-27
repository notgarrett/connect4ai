pub const WIDTH: i32 = 7;
pub const HEIGHT: i32 = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Player1 = 1,
    Player2 = 2,
    Empty = 3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Player1Win,
    Player2Win,
    Draw,
    Ongoing,
}

#[derive(Clone, Copy)]
pub enum Turns {
    Player1Turn,
    Player2Turn,
}

#[derive(Clone, Copy)]
pub struct Game {
    board: [Cell; (WIDTH * HEIGHT) as usize],
    state: GameState,
    turn: Turns,
    count: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [Cell::Empty; (WIDTH * HEIGHT) as usize],
            state: GameState::Ongoing,
            turn: Turns::Player1Turn,
            count: 0,
        }
    }

    pub fn can_play(&self, col: usize) -> (bool, usize) {
        if let GameState::Ongoing = self.state {
        } else {
            return (false, 0);
        }

        for x in 0..=5 {
            if let Cell::Empty = self.board[col + (x * 7) as usize] {
                return (true, col + (x * 7) as usize);
            }
        }

        (false, 0)
    }

    pub fn play(&mut self, col: usize) -> GameState {
        let (playable, position) = self.can_play(col);

        if playable == false {
            return GameState::Ongoing;
        }

        match self.turn {
            Turns::Player1Turn => {
                self.board[position] = Cell::Player1;
            }
            Turns::Player2Turn => {
                self.board[position] = Cell::Player2;
            }
        };

        self.count += 1;

        if self.check_win(position) {
            match self.turn {
                Turns::Player1Turn => self.state = GameState::Player1Win,
                _ => self.state = GameState::Player2Win,
            };
        } else if self.count == 42 {
            self.state = GameState::Draw;
        }

        match self.turn {
            Turns::Player1Turn => self.turn = Turns::Player2Turn,
            _ => self.turn = Turns::Player1Turn,
        };

        self.state.clone()
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn is_winning_move(&self, col: usize) -> bool {
        let mut test = self.clone();
        if let GameState::Player1Win | GameState::Player2Win = test.play(col) {
            true
        } else {
            false
        }
    }

    fn check_win(&self, pos: usize) -> bool {
        self.check_win_horizontal(pos)
            || self.check_win_vertical(pos)
            || self.check_win_diagonal(pos)
    }

    fn check_win_vertical(&self, pos: usize) -> bool {
        if pos < (3 * WIDTH) as usize {
            return false;
        }
        let mut count = 0;
        let iter = 1..4;
        iter.for_each(|iteration| {
            if self.board[pos] == self.board[pos - WIDTH as usize * iteration] {
                count += 1;
            }
        });

        if count == 3 {
            true
        } else {
            false
        }
    }

    fn check_win_horizontal(&self, pos: usize) -> bool {
        let mut count = 0;
        let row = self.get_row(pos);
        // Plan to incorperate this method in when I am not so lazy and decide to optimize this
        // function, until then, I can start working on the algorithm, the (hopefully) fun part.

        // let upper_bound = row as i32 * WIDTH - 1;
        // let lower_bound = (row as i32 - 1) * WIDTH + 1;

        // I want to do a functional or iteration based approach here, but I am struggling to stop
        // the control flow. Will have to look into it.
        for iteration in 1..WIDTH as usize {
            if self.board[pos] == self.board[iteration + WIDTH as usize * row] {
                count += 1;
                if count == 4 {
                    return true;
                }
            } else {
                count = 1;
                if iteration > 4 {
                    return false;
                }
            }
        }

        false
    }

    fn check_win_diagonal(&self, pos: usize) -> bool {
        self.check_win_diagonal_left(pos) || self.check_win_diagonal_right(pos)
    }

    fn check_win_diagonal_left(&self, pos: usize) -> bool {
        let mut count = 0;

        let row = self.get_row(pos);
        let upper_bound = row as i32 * WIDTH - 1;
        let lower_bound = (row as i32 - 1) * WIDTH + 1;

        if pos as i32 + 3 <= upper_bound {
            for iteration in 1..4 {
                let evaluated_position = pos as i32 + WIDTH * iteration + iteration;
                if evaluated_position > (WIDTH * HEIGHT) - 1 {
                    break;
                }

                if self.board[pos] == self.board[evaluated_position as usize] {
                    count += 1
                } else {
                    break;
                }
            }
        } else if pos as i32 - 3 >= lower_bound {
            for iteration in 1..4 {
                let evaluated_position = pos as i32 - WIDTH * iteration - iteration;
                if evaluated_position < 0 {
                    break;
                }

                if self.board[pos] == self.board[evaluated_position as usize] {
                    count += 1
                } else {
                    break;
                }
            }
        }

        if count >= 3 {
            true
        } else {
            false
        }
    }

    fn check_win_diagonal_right(&self, pos: usize) -> bool {
        let mut count = 0;

        let row = self.get_row(pos);
        let upper_bound = row as i32 * WIDTH - 1;
        let lower_bound = (row as i32 - 1) * WIDTH + 1;

        if pos as i32 + 3 >= upper_bound {
            for iteration in 1..4 {
                let evaluated_position = pos as i32 + WIDTH * iteration - iteration;
                if evaluated_position > (WIDTH * HEIGHT) - 1 {
                    break;
                }

                if self.board[pos] == self.board[evaluated_position as usize] {
                    count += 1
                } else {
                    break;
                }
            }
        } else if pos as i32 - 3 <= lower_bound {
            for iteration in 1..4 {
                let evaluated_position = pos as i32 - WIDTH * iteration + iteration;
                if evaluated_position < 0 {
                    break;
                }

                if self.board[pos] == self.board[evaluated_position as usize] {
                    count += 1
                } else {
                    break;
                }
            }
        }

        if count >= 3 {
            true
        } else {
            false
        }
    }

    fn get_row(&self, pos: usize) -> usize {
        pos / WIDTH as usize
    }

    pub fn display_board(&self) {
        for row in (0..=5).rev() {
            for col in 0..=6 {
                match self.board[col + (WIDTH * row) as usize] {
                    Cell::Player1 => print!("1 "),
                    Cell::Player2 => print!("2 "),
                    Cell::Empty => print!("x "),
                };
            }
            println!();
        }
        println!();
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut game = Game::new();

        value.chars().for_each(|col| {
            // This turns the column value into a base 10 digit, then converts it to a usize. You
            // could also do "col as usize - '0' as usize", which is a fancy ascii hack.
            game.play(col.to_digit(10).unwrap() as usize);
        });

        game
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameState};

    #[test]
    fn instantiate_game() {
        let mut x = Game::new();
        assert_eq!(x.can_play(1), (true, 1));
        assert_eq!(x.play(1), GameState::Ongoing);
    }

    #[test]
    fn check_win_veritcal() {
        let mut x = Game::from("121212");
        assert_eq!(x.play(1), GameState::Player1Win);
    }

    #[test]
    fn check_win_horizontal() {
        let mut x = Game::from("112233");
        assert_eq!(x.play(4), GameState::Player1Win);
    }

    #[test]
    fn instantiate_from_string() {
        let _x = Game::from("1554323221");
    }

    #[test]
    fn check_win_diagonals() {
        let mut x = Game::from("0112232335");
        assert_eq!(x.play(3), GameState::Player1Win);
    }

    #[test]
    fn fill_board() {
        let mut x = Game::from("123456123456123456");
        x.play(2);
    }
}
