use crate::bitboard::Bitboard;

pub const WIDTH: i32 = 7;
pub const HEIGHT: i32 = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Player1,
    Player2,
    Empty,
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

// The top most 1(s) will always reflect that the cell under it is a player 2 cell.

#[derive(Clone, Copy)]
pub struct Game {
    bit_board: Bitboard,
    board: [Cell; (WIDTH * HEIGHT) as usize],
    state: GameState,
    turn: Turns,
    count: usize,
}

#[derive(Debug)]
pub enum GameError {
    CannotPlay,
    OutOfBounds,
}

impl Game {
    pub fn new() -> Self {
        Self {
            bit_board: Bitboard::new(),
            board: [Cell::Empty; (WIDTH * HEIGHT) as usize],
            state: GameState::Ongoing,
            turn: Turns::Player1Turn,
            count: 0,
        }
    }

    pub fn can_play(&self, col: usize) -> Option<usize> {
        if let GameState::Ongoing = self.state {
        } else {
            return None;
        }

        for x in 0..HEIGHT as usize {
            if let Cell::Empty = self.board[col + (x * 7)] {
                return Some(col + (x * 7));
            }
        }

        None
    }

    pub fn can_bb_play(&self, col: usize) -> Option<usize> {
        if let GameState::Ongoing = self.state {
        } else {
            return None;
        }

        match self.bit_board.get_bottom(col) {
            Some(x) => Some(x),
            None => None,
        }
    }

    pub fn play(&mut self, col: usize) -> Result<GameState, GameError> {
        if col > (WIDTH * HEIGHT) as usize {
            return Err(GameError::OutOfBounds);
        }

        let position = match self.can_play(col) {
            Some(x) => x,
            None => return Err(GameError::CannotPlay),
        };

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

        Ok(self.state)
    }

    pub fn bb_play(&mut self, col: usize) -> Result<GameState, GameError> {
        if col > (WIDTH * HEIGHT) as usize {
            return Err(GameError::OutOfBounds);
        }

        let position = match self.can_bb_play(col) {
            Some(x) => x,
            None => return Err(GameError::CannotPlay),
        };

        match self.turn {
            Turns::Player1Turn => {
                self.bit_board.set(position, true).unwrap();
            }
            Turns::Player2Turn => {
                self.bit_board.set(position, false).unwrap();
            }
        };

        self.bit_board.set_top(position + WIDTH as usize).unwrap();

        self.count += 1;

        if self.count == 42 {
            self.state = GameState::Draw;
            return Ok(self.state);
        }

        if self.check_bb_win(position) {
            match self.turn {
                Turns::Player1Turn => self.state = GameState::Player1Win,
                _ => self.state = GameState::Player2Win,
            };
        }

        match self.turn {
            Turns::Player1Turn => self.turn = Turns::Player2Turn,
            _ => self.turn = Turns::Player1Turn,
        };

        Ok(self.state)
    }
    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn is_winning_move(&self, col: usize) -> bool {
        let mut test = *self;
        matches!(
            test.play(col),
            Ok(GameState::Player1Win) | Ok(GameState::Player2Win)
        )
    }

    pub fn is_bb_winning_move(&self, col: usize) -> bool {
        let mut test = *self;
        matches!(
            test.bb_play(col),
            Ok(GameState::Player1Win) | Ok(GameState::Player2Win)
        )
    }

    fn check_win(&self, pos: usize) -> bool {
        self.check_win_horizontal(pos)
            || self.check_win_vertical(pos)
            || self.check_win_diagonal(pos)
    }

    fn check_bb_win(&self, pos: usize) -> bool {
        self.check_bb_win_vertical(pos)
            || self.check_bb_win_diagonal(pos)
            || self.check_bb_win_horizontal(pos)
    }

    fn check_win_vertical(&self, pos: usize) -> bool {
        if pos < (3 * WIDTH) as usize {
            return false;
        }
        let mut count = 0;
        for iteration in 1..4 {
            if self.board[pos] == self.board[pos - WIDTH as usize * iteration] {
                count += 1;
            } else {
                return false;
            }
        }

        count >= 3
    }

    fn check_bb_win_vertical(&self, pos: usize) -> bool {
        if pos < (3 * WIDTH) as usize {
            return false;
        }

        let board = match self.turn {
            Turns::Player1Turn => self.bit_board.and(),
            Turns::Player2Turn => self.bit_board.xor(),
        };

        let mut count = 0;
        for iteration in 1..4 {
            if board.get(pos - WIDTH as usize * iteration) {
                count += 1;
            } else {
                return false;
            }
        }

        count >= 3
    }

    fn check_win_horizontal(&self, pos: usize) -> bool {
        let mut count = 0;
        let row = self.get_row(pos);

        for iteration in 0..WIDTH as usize {
            if self.board[pos] == self.board[iteration + WIDTH as usize * row] {
                count += 1;
                if count == 4 {
                    return true;
                }
            } else {
                count = 0;
                if iteration > 4 {
                    return false;
                }
            }
        }

        false
    }

    fn check_bb_win_horizontal(&self, pos: usize) -> bool {
        let mut count = 0;
        let row = self.get_row(pos);

        let board = match self.turn {
            Turns::Player1Turn => self.bit_board.and(),
            Turns::Player2Turn => self.bit_board.xor(),
        };

        for iteration in 0..WIDTH as usize {
            if board.get(iteration + WIDTH as usize * row) {
                count += 1;
                if count == 4 {
                    return true;
                }
            } else {
                count = 0;
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

    fn check_bb_win_diagonal(&self, pos: usize) -> bool {
        self.check_bb_win_diagonal_left(pos) || self.check_bb_win_diagonal_right(pos)
    }

    fn check_win_diagonal_left(&self, pos: usize) -> bool {
        let mut count = 0;

        let row = self.get_row(pos);
        let upper_bound = row as i32 * WIDTH - 1 + WIDTH;
        let lower_bound = (row as i32 - 1) * WIDTH + WIDTH;

        for iteration in 1..4 {
            if pos as i32 + iteration > upper_bound {
                break;
            }

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
        for iteration in 1..4 {
            if pos as i32 - iteration < lower_bound {
                break;
            }

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

        count >= 3
    }

    fn check_bb_win_diagonal_left(&self, pos: usize) -> bool {
        let board = match self.turn {
            Turns::Player1Turn => self.bit_board.and(),
            Turns::Player2Turn => self.bit_board.xor(),
        };

        let mut count = 0;

        let row = self.get_row(pos);
        let upper_bound = row as i32 * WIDTH - 1 + WIDTH;
        let lower_bound = (row as i32 - 1) * WIDTH + WIDTH;

        for iteration in 1..4 {
            if pos as i32 + iteration > upper_bound {
                break;
            }

            let evaluated_position = pos as i32 + WIDTH * iteration + iteration;
            if evaluated_position > (WIDTH * HEIGHT) - 1 {
                break;
            }

            if board.get(evaluated_position as usize) {
                count += 1
            } else {
                break;
            }
        }
        for iteration in 1..4 {
            if pos as i32 - iteration < lower_bound {
                break;
            }

            let evaluated_position = pos as i32 - WIDTH * iteration - iteration;
            if evaluated_position < 0 {
                break;
            }

            if board.get(evaluated_position as usize) {
                count += 1
            } else {
                break;
            }
        }

        count >= 3
    }

    fn check_win_diagonal_right(&self, pos: usize) -> bool {
        let mut count = 0;

        let row = self.get_row(pos);
        let upper_bound = row as i32 * WIDTH - 1 + WIDTH;
        let lower_bound = (row as i32 - 1) * WIDTH + WIDTH;

        for iteration in 1..4 {
            if pos as i32 + iteration < lower_bound {
                break;
            }
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
        for iteration in 1..4 {
            if pos as i32 - iteration > upper_bound {
                break;
            }
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

        count >= 3
    }

    fn check_bb_win_diagonal_right(&self, pos: usize) -> bool {
        let board = match self.turn {
            Turns::Player1Turn => self.bit_board.and(),
            Turns::Player2Turn => self.bit_board.xor(),
        };

        let mut count = 0;

        let row = self.get_row(pos);
        let upper_bound = row as i32 * WIDTH - 1 + WIDTH;
        let lower_bound = (row as i32 - 1) * WIDTH + WIDTH;

        for iteration in 1..4 {
            if pos as i32 + iteration < lower_bound {
                break;
            }
            let evaluated_position = pos as i32 + WIDTH * iteration - iteration;
            if evaluated_position > (WIDTH * HEIGHT) - 1 {
                break;
            }

            if board.get(evaluated_position as usize) {
                count += 1
            } else {
                break;
            }
        }
        for iteration in 1..4 {
            if pos as i32 - iteration > upper_bound {
                break;
            }
            let evaluated_position = pos as i32 - WIDTH * iteration + iteration;
            if evaluated_position < 0 {
                break;
            }

            if board.get(evaluated_position as usize) {
                count += 1
            } else {
                break;
            }
        }

        count >= 3
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
            game.play(col.to_digit(10).unwrap() as usize).unwrap();

            let _ = game.bb_play(col.to_digit(10).unwrap() as usize);
        });

        game
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameState};
    #[test]
    fn instantiate_from_string() {
        let _x = Game::from("1554323221");
    }

    #[test]
    fn check_win_diagonals() {
        let mut x = Game::from("0112232335");
        let mut y = Game::from("5443323220");

        assert_eq!(x.play(3).unwrap(), GameState::Player1Win);

        assert_eq!(y.play(2).unwrap(), GameState::Player1Win);
    }

    #[test]
    fn fill_board() {
        let mut x = Game::from("123456123456123456");
        x.play(2).unwrap();
    }

    #[test]
    fn instantiate_bb_game() {
        let mut x = Game::new();
        assert_eq!(x.can_bb_play(1).unwrap(), 1);
        assert_eq!(x.bb_play(1).unwrap(), GameState::Ongoing);
    }

    #[test]
    fn check_bb_win_veritcal() {
        let mut x = Game::from("121212");
        assert_eq!(x.bb_play(1).unwrap(), GameState::Player1Win);
    }

    #[test]
    fn check_bb_win_horizontal() {
        let mut x = Game::from("112233");
        assert_eq!(x.bb_play(4).unwrap(), GameState::Player1Win);
    }

    #[test]
    fn instantiate_bb_from_string() {
        let _x = Game::from("1554323221");
    }

    #[test]
    fn check_bb_win_diagonals() {
        let mut x = Game::from("0112232335");
        let mut y = Game::from("5443323220");

        assert_eq!(x.bb_play(3).unwrap(), GameState::Player1Win);

        assert_eq!(y.bb_play(2).unwrap(), GameState::Player1Win);
    }

    #[test]
    fn fill_bb_board() {
        let mut x = Game::from("123456123456123456");
        x.bb_play(2).unwrap();
    }
}
