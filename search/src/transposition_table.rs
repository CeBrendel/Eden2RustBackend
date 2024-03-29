
use generic_magic::Bool;
use crate::{I32_NAN, MATE_EVALUATION};

use crate::traits::AlphaBetaSearchFunctionality;

/*
TODO:
    - handle querying here? (What did I mean by this???)
    - remove old entries?
*/


struct TranspositionTableEntry<Board: AlphaBetaSearchFunctionality> {
    pub zobrist_hash: Board::ZobristHash,
    pub depth_left: u8,
    pub evaluation: i32,
    pub is_exact: bool,
    pub is_alpha_cut: bool,
    pub is_beta_cut: bool,
    pub maybe_pv_move: Option<Board::Move>
}


enum EntryVariant<T> {
    None,
    FromAlphaBeta(T),
    FromQuiescence(T)
}


pub struct TranspositionTable<Board: AlphaBetaSearchFunctionality> {
    memory: Vec<EntryVariant<TranspositionTableEntry<Board>>>,
    capacity: usize,
    number_entries: usize
}


impl<Board: AlphaBetaSearchFunctionality> TranspositionTable<Board> {

    const DEFAULT_CAPACITY: usize = 2 << 22;  // 2^22 ~ 4_000_000

    pub fn new() -> Self {
        let mut memory = Vec::with_capacity(Self::DEFAULT_CAPACITY);
        for _hash in 0..Self::DEFAULT_CAPACITY {
            memory.push(EntryVariant::None)
        }
        return Self {memory, capacity: Self::DEFAULT_CAPACITY, number_entries: 0};
    }

    pub fn set_capacity_to(self: &mut Self, capacity: usize) {
        // update the size of the transposition table

        if self.capacity > capacity {
            // take first N elements of memory
            let diff = self.capacity - capacity;
            self.capacity = capacity;
            for _ in 0..diff {
                self.memory.pop();
            }
        } else {
            // add some empty elements to memory
            let diff = capacity - self.capacity;
            self.capacity = capacity;
            for _ in 0..diff {
                self.memory.push(EntryVariant::None);
            }
        }
    }

    #[inline(always)]
    fn index_from_hash(self: &Self, zobrist_hash: Board::ZobristHash) -> usize {
        let hash_as_usize: usize = unsafe {
            std::mem::transmute_copy(&zobrist_hash)
        };  // TODO
        hash_as_usize % self.capacity
    }

    pub(crate) fn put<
        FromAlphaBeta: Bool,
        FromQuiescence: Bool
    >(
        self: &mut Self, board: &Board,
        depth_left: u8,
        distance_to_root: i32,
        mut evaluation: i32,
        is_exact: bool, is_alpha_cut: bool, is_beta_cut: bool,
        maybe_pv_move: Option<Board::Move>
    ) {

        // remove distance-to-root offset from mate score
        if evaluation > MATE_EVALUATION / 2 {
            evaluation += distance_to_root;
            assert!(evaluation <= MATE_EVALUATION);
        } else if evaluation < -MATE_EVALUATION / 2 {
            evaluation -= distance_to_root;
            assert!(evaluation >= - MATE_EVALUATION);
        }

        // find index
        let zobrist_hash = board.zobrist_hash();
        let index = self.index_from_hash(zobrist_hash);

        // construct entry
        let entry = TranspositionTableEntry{
            zobrist_hash,
            depth_left,
            evaluation,
            is_exact,
            is_alpha_cut,
            is_beta_cut,
            maybe_pv_move,
        };

        // store entry
        self.number_entries = usize::min(self.capacity, self.number_entries + 1);
        if FromAlphaBeta::AS_BOOL {
            self.memory[index] = EntryVariant::FromAlphaBeta(entry);
        } else if FromQuiescence::AS_BOOL {
            self.memory[index] = EntryVariant::FromQuiescence(entry);
        }
    }

    pub(crate) fn query<
        CalledInQuiescence: Bool
    >(
        self: &mut Self,
        board: &Board,
        mut alpha: i32,
        mut beta: i32,
        depth_left: u8,
        distance_to_root: i32
    ) -> (bool, bool, i32, Option<Board::Move>) {
        // query if given board is in transposition table and if it contains useful information, return it

        let mut is_hit: bool = false;
        let mut is_exact: bool = false;
        let mut evaluation: i32 = I32_NAN;
        let mut maybe_pv_move: Option<Board::Move> = None;

        let index = self.index_from_hash(board.zobrist_hash());
        match &self.memory[index] {
            EntryVariant::None => {},

            EntryVariant::FromAlphaBeta(entry) => 'arm: {
                if (CalledInQuiescence::AS_BOOL || (entry.depth_left >= depth_left)) &&
                    entry.zobrist_hash == board.zobrist_hash() {

                    // copy entry.evaluation
                    let mut entry_evaluation = entry.evaluation;

                    // add mate depth offset
                    if entry_evaluation > MATE_EVALUATION/2 {
                        entry_evaluation -= distance_to_root;
                        assert!(entry_evaluation <= MATE_EVALUATION);
                    } else if entry_evaluation < -MATE_EVALUATION/2 {
                        entry_evaluation += distance_to_root;
                        assert!(entry_evaluation >= -MATE_EVALUATION);
                    }

                    // check whether entry has an exact evaluation, if so return
                    if entry.is_exact {
                        is_hit = true;
                        is_exact = true;
                        evaluation = entry_evaluation;
                        maybe_pv_move = entry.maybe_pv_move;
                        break 'arm;
                    }

                    // update bounds
                    if entry.is_alpha_cut {
                        // entry_evaluation is an upper bound
                        beta = i32::min(beta, entry_evaluation);
                    } else if entry.is_beta_cut {
                        // entry_evaluation is a lower bound
                        alpha = i32::max(alpha, entry_evaluation);
                    }

                    // check for cut-off
                    if alpha >= beta {
                        is_hit = true;
                        is_exact = false;
                        evaluation = entry_evaluation;
                        maybe_pv_move = entry.maybe_pv_move;
                    }
                };
            },

            EntryVariant::FromQuiescence(entry) => 'arm: {
                if CalledInQuiescence::AS_BOOL && entry.zobrist_hash == board.zobrist_hash() {

                    // copy entry.evaluation
                    let mut entry_evaluation = entry.evaluation;

                    // add mate depth offset
                    if entry_evaluation > MATE_EVALUATION/2 {
                        entry_evaluation -= distance_to_root;
                        assert!(entry_evaluation <= MATE_EVALUATION);
                    } else if entry_evaluation < -MATE_EVALUATION/2 {
                        entry_evaluation += distance_to_root;
                        assert!(entry_evaluation >= -MATE_EVALUATION);
                    }

                    // check whether entry has an exact evaluation, if so return
                    if entry.is_exact {
                        is_hit = true;
                        is_exact = true;
                        evaluation = entry_evaluation;
                        maybe_pv_move = entry.maybe_pv_move;
                        break 'arm;
                    }

                    // update bounds
                    if entry.is_alpha_cut {
                        // entry_evaluation is an upper bound
                        beta = i32::min(beta, entry_evaluation);
                    } else if entry.is_beta_cut {
                        // entry_evaluation is a lower bound
                        alpha = i32::max(alpha, entry_evaluation);
                    }

                    // check for cut-off
                    if alpha >= beta {
                        is_hit = true;
                        is_exact = false;
                        evaluation = entry_evaluation;
                        maybe_pv_move = entry.maybe_pv_move;
                    }
                }
            }
        };

        return (is_hit, is_exact, evaluation, maybe_pv_move)
    }

    pub fn get_pv_line(self: &Self, board: &mut Board) -> Vec<Board::Move> {
        // extract the pv line from the transposition table

        let mut n_moves_found: usize = 0;
        let mut moves = Vec::with_capacity(16);
        let mut seen_hashes = Vec::with_capacity(16);

        loop {
            let current_hash = board.zobrist_hash();
            let current_index = self.index_from_hash(current_hash);

            // break loops
            if seen_hashes.contains(&current_hash) {
                println!("Loop in PV extraction from transposition table!");
                break;
            }

            match &self.memory[current_index] {
                EntryVariant::None => break,
                EntryVariant::FromQuiescence(_) => break, // TODO: We could extract moves here...
                EntryVariant::FromAlphaBeta(entry) => {
                    if entry.zobrist_hash == current_hash {
                        match entry.maybe_pv_move {
                            None => break,
                            Some(r#move) => {
                                n_moves_found += 1;
                                moves.push(r#move);
                                seen_hashes.push(current_hash);
                                board.make_move(r#move);
                            }
                        }
                    } else {
                        break
                    }
                }
            }
        }

        for _ in 0..n_moves_found {
            board.unmake_move();
        }

        return moves;
    }

    pub fn fill_level_per_mill(self: &Self) -> usize {
        (1000. * (self.number_entries as f32) / (self.capacity as f32)) as usize
    }
}
