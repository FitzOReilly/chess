use eval::eval::{Eval, Score};
use movegen::move_generator::MoveGenerator;
use movegen::position_history::PositionHistory;
use movegen::r#move::Move;
use movegen::r#move::MoveList;
use movegen::side::Side;

pub struct Search;

impl Search {
    pub fn negamax(pos_history: &mut PositionHistory, depth: usize) -> (Score, MoveList) {
        let mut move_list_stack = vec![MoveList::new(); depth];

        let pv_size = depth * (depth + 1) / 2;
        let mut principal_variation = MoveList::with_capacity(pv_size);
        principal_variation.resize(pv_size, Move::NULL);
        let eval = match pos_history.current_pos().side_to_move() {
            Side::White => Self::negamax_recursive(
                &mut move_list_stack,
                &mut principal_variation,
                pos_history,
                depth,
            ),
            Side::Black => -Self::negamax_recursive(
                &mut move_list_stack,
                &mut principal_variation,
                pos_history,
                depth,
            ),
        };

        principal_variation.truncate(depth);
        (eval, principal_variation)
    }

    pub fn alpha_beta(pos_history: &mut PositionHistory, depth: usize) -> (Score, MoveList) {
        let mut move_list_stack = vec![MoveList::new(); depth];

        let pv_size = depth * (depth + 1) / 2;
        let mut principal_variation = MoveList::with_capacity(pv_size);
        principal_variation.resize(pv_size, Move::NULL);
        let alpha = Score::MIN + 1;
        let beta = Score::MAX;
        let eval = match pos_history.current_pos().side_to_move() {
            Side::White => Self::alpha_beta_recursive(
                &mut move_list_stack,
                &mut principal_variation,
                pos_history,
                alpha,
                beta,
                depth,
            ),
            Side::Black => -Self::alpha_beta_recursive(
                &mut move_list_stack,
                &mut principal_variation,
                pos_history,
                -beta,
                -alpha,
                depth,
            ),
        };

        principal_variation.truncate(depth);
        (eval, principal_variation)
    }

    fn negamax_recursive(
        move_list_stack: &mut Vec<MoveList>,
        principal_variation: &mut MoveList,
        pos_history: &mut PositionHistory,
        depth: usize,
    ) -> Score {
        let mut max = Score::MIN;

        // TODO Also check terminal nodes
        match depth {
            0 => max = Eval::eval_relative(pos_history.current_pos()),
            _ => {
                debug_assert!(!move_list_stack.is_empty());
                let mut move_list = move_list_stack.pop().unwrap();
                MoveGenerator::generate_moves(&mut move_list, pos_history.current_pos());
                for m in move_list.iter() {
                    pos_history.do_move(*m);
                    let new_score = -Self::negamax_recursive(
                        move_list_stack,
                        principal_variation,
                        pos_history,
                        depth - 1,
                    );
                    if new_score > max {
                        max = new_score;
                        let best_move = *m;

                        let dist_from_end = depth * (depth + 1) / 2;
                        let idx = principal_variation.len() - dist_from_end;
                        principal_variation[idx] = best_move;
                        for i in 1..depth {
                            principal_variation[idx + i] = principal_variation[idx + i + depth - 1];
                        }
                    }
                    pos_history.undo_last_move();
                }
                move_list_stack.push(move_list);
            }
        }
        max
    }

    fn alpha_beta_recursive(
        move_list_stack: &mut Vec<MoveList>,
        principal_variation: &mut MoveList,
        pos_history: &mut PositionHistory,
        mut alpha: Score,
        beta: Score,
        depth: usize,
    ) -> Score {
        let mut score = Score::MIN + 1;

        // TODO Also check terminal nodes
        match depth {
            0 => score = Eval::eval_relative(pos_history.current_pos()),
            _ => {
                debug_assert!(!move_list_stack.is_empty());
                let mut move_list = move_list_stack.pop().unwrap();
                MoveGenerator::generate_moves(&mut move_list, pos_history.current_pos());
                for m in move_list.iter() {
                    pos_history.do_move(*m);
                    let new_score = -Self::alpha_beta_recursive(
                        move_list_stack,
                        principal_variation,
                        pos_history,
                        -beta,
                        -alpha,
                        depth - 1,
                    );
                    if new_score >= beta {
                        score = beta;
                        pos_history.undo_last_move();
                        break;
                    }
                    if new_score > alpha {
                        alpha = new_score;
                        score = new_score;
                        let best_move = *m;

                        let dist_from_end = depth * (depth + 1) / 2;
                        let idx = principal_variation.len() - dist_from_end;
                        principal_variation[idx] = best_move;
                        for i in 1..depth {
                            principal_variation[idx + i] = principal_variation[idx + i + depth - 1];
                        }
                    }
                    pos_history.undo_last_move();
                }
                move_list_stack.push(move_list);
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::position::Position;
    use movegen::position_history::PositionHistory;

    #[test]
    fn negamax() {
        let mut pos_history = PositionHistory::new(Position::initial());

        for depth in 0..=3 {
            let (score, pv) = Search::negamax(&mut pos_history, depth);

            for m in pv.iter() {
                pos_history.do_move(*m);
            }
            assert_eq!(Eval::eval(pos_history.current_pos()), score);
            for _ in 0..depth {
                pos_history.undo_last_move();
            }
        }
    }

    #[test]
    fn alpha_beta() {
        let mut pos_history = PositionHistory::new(Position::initial());

        for depth in 0..=4 {
            let (score, pv) = Search::alpha_beta(&mut pos_history, depth);

            for m in pv.iter() {
                pos_history.do_move(*m);
            }
            assert_eq!(Eval::eval(pos_history.current_pos()), score);
            for _ in 0..depth {
                pos_history.undo_last_move();
            }
        }
    }
}
