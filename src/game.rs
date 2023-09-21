// Connect 4 sucks ass
//

const WIDTH: i32 = 7;
const HEIGHT: i32 = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Player1 = 1,
    Player2 = 2,
    Empty = 3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameState {
    Player1Win,
    Player2Win,
    Draw,
    Ongoing,
}

#[derive(Clone, Copy)]
enum Turns {
    Player1Turn,
    Player2Turn,
}

#[derive(Clone, Copy)]
struct Game {
    board: [Cell; (WIDTH * HEIGHT) as usize],
    state: GameState,
    turn: Turns,
    count: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [Cell::Empty; (WIDTH * HEIGHT) as usize],
            state: GameState::Ongoing,
            turn: Turns::Player1Turn,
            count: 0,
        }
    }

    fn can_play(&self, col: usize) -> (bool, usize) {
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

    fn play(&mut self, col: usize) -> GameState {
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

    fn is_winning_move(&self, col: usize) -> bool {
        let mut test = self.clone();
        if let GameState::Player1Win | GameState::Player2Win = test.play(col) {
            return true;
        }
        false
    }

    fn check_win(&self, pos: usize) -> bool {
        self.check_win_horizontal(pos)
            || self.check_win_vertical(pos)
            || self.check_win_diagonal(pos)
    }

    fn check_win_vertical(&self, pos: usize) -> bool {
        if pos < ((3 * WIDTH) - 1) as usize {
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
        false
    }

    fn get_row(&self, pos: usize) -> usize {
        pos / WIDTH as usize
    }

    fn get_bottom_possible_diagonal_left(&self, pos: usize) -> usize {
        todo!()
    }

    fn get_bottom_possible_diagonal_right(&self, pos: usize) -> usize {
        todo!()
    }

    fn display_board(&self) {
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
        x.display_board();
        assert_eq!(x.play(1), GameState::Player1Win);
    }

    #[test]
    fn check_win_horizontal() {
        let mut x = Game::from("112233");
        x.display_board();
        assert_eq!(x.play(4), GameState::Player1Win);
    }

    #[test]
    fn instantiate_from_string() {
        let x = Game::from("1554323221");
        x.display_board();
    }
}
