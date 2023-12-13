
use generic_magic::Bool;
use crate::traits::SearchableMove;

const EAGER: bool = true;


const PV_SCORE: i32 = 2000000;
const RECAPTURE_SCORE: i32 = 2000000 - 1;
const CAPTURE_SCORE: i32 = 2000000 - 2;

static MVV_LVA_SCORES: [[i32; 12]; 12] = {
    // for each pair (victim, attacker) of pieces the MVV-LVA score given by arr[victim][attacker]

    let mut scores = [[0; 12]; 12];

    let capture_bonus: i32 = 10;  // positive: captures before non-captures, negative: non-captures before captures of score 0
    let piece_values: [i32; 6] = [100, 300, 325, 500, 900, i32::MAX/3];

    let mut victim: usize = 0;
    while victim < 12 {

        let mut attacker: usize = 0;
        while attacker < 12 {

            let victim_value = piece_values[victim % 6];
            let attacker_value = piece_values[victim % 6];
            scores[victim][attacker] = victim_value - attacker_value + capture_bonus;

            attacker += 1;
        }

        victim += 1;
    }

    scores
};


struct MoveScorePair<Move>{
    r#move: Move,
    score: i32
}

pub struct MoveList<Move: SearchableMove>{
    pairs: Vec<MoveScorePair<Move>>,
    searched: usize
}


impl<Move: SearchableMove> MoveList<Move> {
    pub(crate) fn new<OnlyLoud: Bool, HasLastMove: Bool>(
        legal_moves: Vec<Move>,
        maybe_pv_move: Option<Move>,
        maybe_last_move: Option<Move>,
        history_heuristic: &[[i32; 64]; 12]
    ) -> Self {

        // TODO: Maybes are inefficient. Add generics.

        // buffers for (re)capture heuristic
        let mut recapture_lva_index: Option<usize> = None;
        let mut recapture_lva_as_index: usize = usize::MAX;
        let recapture_target = if HasLastMove::AS_BOOL {
            maybe_last_move.unwrap().to_square_as_index()
        } else {
            usize::MAX  // invalid square!
        };

        // pv move buffer
        let mut pv_move_index: Option<usize> = None;

        // construct MoveScorePairs
        let mut pairs: Vec<MoveScorePair<Move>> = Vec::with_capacity(legal_moves.len());
        for (index, r#move) in legal_moves.into_iter().enumerate() {

            // remember index of pv move
            if Some(r#move) == maybe_pv_move {
                pv_move_index = Some(index);
            }

            let moving_piece = r#move.moving_piece_as_index();
            let to_square = r#move.to_square_as_index();

            let mut score: i32 = 0;

            // loud moves
            if OnlyLoud::AS_BOOL || r#move.is_loud() {

                // keep track of LVA for (re)capture heuristic
                if HasLastMove::AS_BOOL && to_square == recapture_target {
                    if moving_piece < recapture_lva_as_index {  // do we have a new LVA?
                        recapture_lva_as_index = moving_piece;
                        recapture_lva_index = Some(index);
                    }
                }

                // handle MVV-LVA heuristic
                if r#move.is_capture() {
                    let captured_piece = r#move.captured_piece_as_index();
                    score += MVV_LVA_SCORES[captured_piece][moving_piece];
                }

            }

            // quiet moves
            if !OnlyLoud::AS_BOOL && !r#move.is_loud() {  // TODO: This is the else-block of the above if
                // handle history heuristic
                score += history_heuristic[moving_piece][to_square];
            }

            // push to pairs
            pairs.push(MoveScorePair{r#move, score});
        }

        // add (re)capture bonus
        if HasLastMove::AS_BOOL {
            match recapture_lva_index {
                None => {},
                Some(idx) => pairs[idx].score = if maybe_last_move.unwrap().is_capture() {
                    RECAPTURE_SCORE
                } else {
                    CAPTURE_SCORE
                }
            }
        }

        // add pv bonus
        match pv_move_index {
            None => {},
            Some(idx) => pairs[idx].score = PV_SCORE
        }

        // return object
        if EAGER {
            let mut moves = Self{pairs, searched: 0};
            moves.eager_sort();
            return moves;
        } else {
            return Self{pairs, searched: 0};
        }


    }

    pub fn eager_sort(self: &mut Self) {
        self.pairs.sort_unstable_by_key(|pair| pair.score)
    }
}

impl<Move: SearchableMove> Iterator for MoveList<Move> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {

        if EAGER {

            Some(self.pairs.pop()?.r#move)
            /*println!("{}, {}, {}", pair.score, pair.r#move.to_string(), self.pairs.len());*/


        } else {

            // how many moves to search
            let moves_left: usize = self.pairs.len() - self.searched;

            if moves_left == 0 {
                return None;
            }

            // find best move from those not yet searched
            let mut index: usize = 0;
            let mut best_index = None;
            let mut best_move = None;
            let mut best_score = i32::MIN;
            for &MoveScorePair{r#move, score} in &self.pairs {

                if score > best_score {
                    best_index = Some(index);
                    best_move = Some(r#move);
                    best_score = score;
                }

                index += 1;
                if index == moves_left {
                    break
                }
            }

            // swap moves
            match best_index {
                None => {},
                Some(idx) => {
                    self.pairs.swap(idx, moves_left-1);
                    self.searched += 1;

                    /*println!("{}, {}, {}", best_score, self.pairs[moves_left-1].r#move.to_string(), moves_left);*/

                }
            }

            return best_move;

        }
    }
}