use crate::game::{Game, GameState, HEIGHT, WIDTH};

pub fn negamax(position: &Game, mut alpha: i32, mut beta: i32) -> i32 {
    if position.state() == GameState::Draw {
        return 0;
    }

    for i in 0..WIDTH as usize {
        if position.can_play(i).0 && position.is_winning_move(i) {
            return (WIDTH * HEIGHT + 1 - position.count() as i32) / 2;
        }
    }

    let max = (WIDTH * HEIGHT - 1 - position.count() as i32) / 2;

    if beta > max {
        beta = max;
        if alpha >= beta {
            return beta;
        }
    }

    let column_order = [3, 4, 5, 2, 1, 6, 0];
    for i in 0..WIDTH as usize {
        if position.can_play(column_order[i]).0 {
            let mut position2 = position.clone();
            position2.play(column_order[i]);
            let score = -negamax(&position2, -beta, -alpha);

            if score >= beta {
                return score;
            }

            if score > alpha {
                alpha = score
            }
        }
    }
    alpha
}

#[cfg(test)]
mod tests {
    use super::negamax;
    use crate::game;

    #[test]
    fn completetion_test() {
        let x = game::Game::from("0011223");
        let y = negamax(&x, -1000000, 1000000);
        println!("{}", y);
    }
}
