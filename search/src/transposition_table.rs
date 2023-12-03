
use generic_magic::Bool;
use crate::I32_NAN;

use crate::traits::AlphaBetaAndQuiescenceSearchFunctionality;

/*
TODO:
    - combine has and get into an option type that should be handled in the search
    - handle querying and pv move insertion here
    - extract pv
    - keep track of fill size
    - remove old entries?
*/


struct TranspositionTableEntry<Board: AlphaBetaAndQuiescenceSearchFunctionality> {
    pub zobrist_hash: Board::ZobristHash,
    pub depth: u8,
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


pub struct TranspositionTable<Board: AlphaBetaAndQuiescenceSearchFunctionality> {
    memory: Vec<EntryVariant<TranspositionTableEntry<Board>>>,
    capacity: usize,
    number_entries: usize
}


impl<Board: AlphaBetaAndQuiescenceSearchFunctionality> TranspositionTable<Board> {

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
        depth: u8,
        evaluation: i32,
        is_exact: bool, is_alpha_cut: bool, is_beta_cut: bool,
        pv_move: Option<Board::Move>
    ) {
        // find index
        let index = self.index_from_hash(board.zobrist_hash());

        // construct entry
        let entry = TranspositionTableEntry{
            zobrist_hash: board.zobrist_hash(),
            depth,
            evaluation,
            is_exact,
            is_alpha_cut,
            is_beta_cut,
            maybe_pv_move: pv_move,
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
        depth: u8
    ) -> (bool, bool, i32, Option<Board::Move>) {
        // query if given board is in transposition table and return is it contains useful information

        let index = self.index_from_hash(board.zobrist_hash());
        match &self.memory[index] {
            EntryVariant::None => {},

            EntryVariant::FromAlphaBeta(entry) => {
                if entry.zobrist_hash == board.zobrist_hash() && (CalledInQuiescence::AS_BOOL || (entry.depth >= depth)) {

                    // check whether entry has an exact evaluation, if so return
                    if entry.is_exact {
                        return (true, true, entry.evaluation, entry.maybe_pv_move);
                    }

                    // update bounds
                    if entry.is_alpha_cut {
                        // entry.evaluation is an upper bound
                        beta = i32::min(beta, entry.evaluation);
                    } else if entry.is_beta_cut {
                        // entry.evaluation is a lower bound
                        alpha = i32::max(alpha, entry.evaluation);
                    }

                    // check for cut-off
                    if alpha >= beta {
                        return (true, false, entry.evaluation, entry.maybe_pv_move);
                    }
                };
            },

            EntryVariant::FromQuiescence(entry) => {
                if entry.zobrist_hash == board.zobrist_hash() && CalledInQuiescence::AS_BOOL {

                    // check whether entry has an exact evaluation, if so return
                    if entry.is_exact {
                        return (true, true, entry.evaluation, entry.maybe_pv_move);
                    }

                    // update bounds
                    if entry.is_alpha_cut {
                        // entry.evaluation is an upper bound
                        beta = i32::min(beta, entry.evaluation);
                    } else if entry.is_beta_cut {
                        // entry.evaluation is a lower bound
                        alpha = i32::max(alpha, entry.evaluation);
                    }

                    // check for cut-off
                    if alpha >= beta {
                        return (true, false, entry.evaluation, entry.maybe_pv_move);
                    }
                }
            }
        };

        // no arm found a good entry
        return (false, false, I32_NAN, None)
    }

    pub fn get_pv_line(self: &Self, board: &mut Board) -> Vec<Board::Move> {
        // extract the pv line from the transposition table

        let mut n_moves_found: usize = 0;
        let mut moves = Vec::with_capacity(16);

        loop {
            let current_hash = board.zobrist_hash();
            let current_index = self.index_from_hash(current_hash);
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
                                board.make_move(r#move);
                            }
                        }
                    }
                }
            }
        }

        for _ in 0..n_moves_found {
            board.unmake_move();
        }

        return moves;
    }

    pub fn fill_level_per_mill(self: &Self) -> f32 {
        (self.number_entries as f32) / (self.capacity as f32)
    }
}
