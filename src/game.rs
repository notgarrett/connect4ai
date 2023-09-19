// Connect 4 sucks ass
//

const WIDTH: i32 = 7;
const HEIGHT: i32 = 6;

#[derive(Clone, Copy)]
enum Cell {
    Player1,
    Player2,
    Empty,
}

#[derive(Clone, Copy)]
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
    fn new(&self) -> Self {
        Self {
            board: [Cell::Empty; (WIDTH * HEIGHT) as usize],
            state: GameState::Ongoing,
            turn: Turns::Player1Turn,
            count: 0,
        }
    }

    fn can_play(&self, col: usize) -> (bool, usize) {
        if let GameState::Ongoing = self.state {
            println!("Game is ongoing!");
        } else {
            return (false, 0);
        }

        for x in 0..5 {
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

        let current_turn = self.turn.clone();

        match self.turn {
            Turns::Player1Turn => {
                self.turn = Turns::Player2Turn;
                self.board[position] = Cell::Player1;
            }
            Turns::Player2Turn => {
                self.turn = Turns::Player1Turn;
                self.board[position] = Cell::Player2;
            }
        };

        self.count += 1;

        if self.check_win() {
            match current_turn {
                Turns::Player1Turn => self.state = GameState::Player1Win,
                _ => self.state = GameState::Player2Win,
            };
        } else if self.count == 42 {
            self.state = GameState::Draw;
        }

        self.state.clone()
    }

    fn is_winning_move(&self, col: usize) -> bool {
        let mut test = self.clone();
        if let GameState::Player1Win | GameState::Player2Win = test.play(col) {
            return true;
        }
        false
    }

    fn check_win(&self) -> bool {
        self.check_win_horizontal() || self.check_win_veritcal() || self.check_win_diagonal()
    }

    fn check_win_horizontal(&self) -> bool {
        unimplemented!()
    }

    fn check_win_veritcal(&self) -> bool {
        unimplemented!()
    }

    fn check_win_diagonal(&self) -> bool {
        unimplemented!()
    }
}
