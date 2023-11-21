
use generic_magic::Bool;

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
    pub evaluation: f32,
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
    capacity: usize
}


impl<Board: AlphaBetaAndQuiescenceSearchFunctionality> TranspositionTable<Board> {

    const DEFAULT_CAPACITY: usize = 2 << 22;  // 2^22 ~ 4_000_000

    pub fn new() -> Self {
        let mut memory = Vec::with_capacity(Self::DEFAULT_CAPACITY);
        for _hash in 0..Self::DEFAULT_CAPACITY {
            memory.push(EntryVariant::None)
        }
        return Self {memory, capacity: Self::DEFAULT_CAPACITY};
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
        evaluation: f32,
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
        if FromAlphaBeta::AS_BOOL {
            self.memory[index] = EntryVariant::FromAlphaBeta(entry);
        } else if FromQuiescence::AS_BOOL {
            self.memory[index] = EntryVariant::FromQuiescence(entry);
        }
    }

    pub(crate) fn query<
        CalledInAlphaBeta: Bool,
        CalledInQuiescence: Bool
    >(
        self: &mut Self,
        board: &Board,
        mut alpha: f32,
        mut beta: f32,
        depth: u8
    ) -> (bool, bool, f32, Option<Board::Move>) {
        // query if given board is in transposition table and return is it contains useful information

        let mut is_hit: bool = false;
        let mut is_exact: bool = false;
        let mut evaluation: f32 = f32::NAN;
        let mut maybe_pv_move: Option<Board::Move> = None;

        // TODO: CalledInQuiescence can use any FromAlphaBeta, irrespective of depth!
        if self.has::<CalledInAlphaBeta>(board) {
            'block: {
                let entry = self.get(board);

                // check for stored value
                if entry.depth >= depth {

                    if entry.is_exact {
                        is_hit = true;
                        is_exact = true;
                        evaluation = entry.evaluation;
                        maybe_pv_move = entry.maybe_pv_move;

                        break 'block;
                    }

                    if entry.is_alpha_cut {
                        // entry.evaluation is an upper bound
                        beta = f32::min(beta, entry.evaluation);
                    } else if entry.is_beta_cut {
                        // entry.evaluation is a lower bound
                        alpha = f32::max(alpha, entry.evaluation);
                    }

                    // check for cut-off
                    if alpha >= beta {
                        is_hit = true;
                        is_exact = false;
                        evaluation = entry.evaluation;
                        maybe_pv_move = entry.maybe_pv_move
                    }
                }
            }
        };

        return (is_hit, is_exact, evaluation, maybe_pv_move);
    }

    fn has<
        CalledInAlphaBeta: Bool
    >(self: &Self, board: &Board) -> bool {
        // find index
        let index = self.index_from_hash(board.zobrist_hash());

        match &self.memory[index] {
            EntryVariant::None => false,
            EntryVariant::FromAlphaBeta(entry) => entry.zobrist_hash == board.zobrist_hash(),
            EntryVariant::FromQuiescence(entry) => !CalledInAlphaBeta::AS_BOOL && (entry.zobrist_hash == board.zobrist_hash())
        }
    }

    fn get(self: &Self, board: &Board) -> &TranspositionTableEntry<Board> {
        let index = self.index_from_hash(board.zobrist_hash());

        match &self.memory[index] {
            EntryVariant::None => panic!("Access to Empty!"),
            EntryVariant::FromAlphaBeta(entry) => entry,
            EntryVariant::FromQuiescence(entry) => entry,
        }
    }

}
