
use crate::traits::AlphaBetaAndQuiescenceSearchFunctionality;

/*
TODO:
    - combine has and get into an option type that should be handled in the search
*/

pub struct TranspositionTableEntry<Board: AlphaBetaAndQuiescenceSearchFunctionality> {
    pub zobrist_hash: Board::ZobristHash,
    pub depth: u8,
    pub evaluation: f32,
    pub is_exact: bool,
    pub is_alpha_cut: bool,
    pub is_beta_cut: bool,
    pub maybe_pv_move: Option<Board::Move>
}


pub struct TranspositionTable<Board: AlphaBetaAndQuiescenceSearchFunctionality> {
    memory: Vec<Option<TranspositionTableEntry<Board>>>,
    capacity: usize
}


impl<Board: AlphaBetaAndQuiescenceSearchFunctionality> TranspositionTable<Board> {

    const DEFAULT_CAPACITY: usize = 2 << 20;  // 2^20 ~ 1_000_000

    pub fn new() -> Self {
        let mut memory = Vec::with_capacity(Self::DEFAULT_CAPACITY);
        for _hash in 0..Self::DEFAULT_CAPACITY {
            memory.push(None)
        }
        return Self {memory, capacity: Self::DEFAULT_CAPACITY};
    }

    /*pub fn set_capacity_to(self: &mut Self, capacity: usize) {
        // update the size of the transposition table, TODO: check if this is correct

        if self.capacity > capacity {
            // take first N elements of memory
            self.capacity = capacity;
            self.memory = self.memory[0..capacity].to_vec();
        } else {
            // add some empty elements to memory
            let diff = capacity - self.capacity;
            self.capacity = capacity;
            for _ in 0..diff {
                self.memory.push(None);
            }
        }
    }*/

    #[inline(always)]
    fn index_from_hash(self: &Self, zobrist_hash: Board::ZobristHash) -> usize {
        let hash_as_usize: usize = unsafe {
            std::mem::transmute_copy(&zobrist_hash)
        };  // TODO
        hash_as_usize % self.capacity
    }

    pub(crate) fn has(self: &Self, board: &Board) -> bool {
        let hash = self.index_from_hash(board.zobrist_hash());
        match &self.memory[hash] {
            None => false,
            Some(entry) => entry.zobrist_hash == board.zobrist_hash()
        }
    }

    pub(crate) fn get(self: &Self, board: &Board) -> &TranspositionTableEntry<Board> {
        let hash = self.index_from_hash(board.zobrist_hash());
        self.memory[hash].as_ref().expect("Access to Empty!")
    }

    pub(crate) fn put(
        self: &mut Self, board: &Board,
        depth: u8,
        evaluation: f32,
        is_exact: bool, is_alpha_cut: bool, is_beta_cut: bool,
        pv_move: Option<Board::Move>
    ) {
        let hash = self.index_from_hash(board.zobrist_hash());
        self.memory[hash] = Some(
            TranspositionTableEntry{
                zobrist_hash: board.zobrist_hash(),
                depth,
                evaluation,
                is_exact,
                is_alpha_cut,
                is_beta_cut,
                maybe_pv_move: pv_move,
            }
        );
    }
}
