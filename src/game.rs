// Connect 4 sucks ass
//

const WIDTH: i32 = 7;
const HEIGHT: i32 = 6;

#[derive(Clone, Copy)]
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
            || self.check_win_veritcal(pos)
            || self.check_win_diagonal(pos)
    }

    fn check_win_horizontal(&self, pos: usize) -> bool {
        unimplemented!()
    }

    fn check_win_veritcal(&self, pos: usize) -> bool {
        unimplemented!()
    }

    fn check_win_diagonal(&self, pos: usize) -> bool {
        unimplemented!()
    }

    fn recursive_check_win_vertical(&self, pos: i32, mut count: usize, turn: Turns) -> bool {
        if pos < 0 {
            return false;
        }

        if count == 4 {
            return true;
        }

        match self.board[pos as usize] {
            Cell::Player1 => {
                if let Turns::Player1Turn = turn {
                    count += 1;
                } else {
                    count = 0;
                }
            }
            Cell::Player2 => {
                if let Turns::Player2Turn = turn {
                    count += 1;
                } else {
                    count = 0;
                }
            }
            _ => count = 0,
        }

        self.recursive_check_win_vertical(pos - 7, count, turn)
    }

    fn display_board(&self) {
        for row in (0..=5).rev() {
            for col in 0..=6 {
                match self.board[col + (7 * row) as usize] {
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

impl From<String> for Game {
    fn from(value: String) -> Self {
        let mut game = Game::new();

        for col in value.chars() {
            game.play(col as usize);
        }

        game
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameState};

    #[test]
    fn instantiate_game() {
        let mut x = Game::new();
        x.display_board();
        assert_eq!(x.can_play(1), (true, 1));
        assert_eq!(x.play(1), GameState::Ongoing);
        x.display_board();
    }

    #[test]
    fn check_win() {}

    #[test]
    fn instantiate_from_string() {}
}
