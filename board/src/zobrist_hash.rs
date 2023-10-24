
/*
TODO:
    - Generate various random keys
    - maybe switch to longer keys? u128 or [u8, N] maybe?
    - hash repetitions?
    - don't hash move counter/repetitions? This will kill a lot of transpositions...
*/

use const_random::const_random;

use crate::pieces::Piece;
use bitboards::squares::Square;

#[derive(Copy, Clone)]
#[derive(PartialEq)]  // TODO: Unnecessary
pub struct ZobristHash(u64);

impl Default for ZobristHash {
    fn default() -> Self {
        Self(0)
    }
}

impl ZobristHash {
    const fn u64_from_bytes(bytes: [u8; 8]) -> u64 {
        return 0
                | (bytes[0] as u64)
                | (bytes[1] as u64) << 8
                | (bytes[2] as u64) << 16
                | (bytes[3] as u64) << 24
                | (bytes[4] as u64) << 32
                | (bytes[5] as u64) << 40
                | (bytes[6] as u64) << 48
                | (bytes[7] as u64) << 56;
    }

    // TODO: Should arrays be static instead of const?
    const PLAYER_KEY: u64 = const_random!(u64);
    const WHITE_SHORT_KEY: u64 = const_random!(u64);
    const WHITE_LONG_KEY: u64 = const_random!(u64);
    const BLACK_SHORT_KEY: u64 = const_random!(u64);
    const BLACK_LONG_KEY: u64 = const_random!(u64);
    const PIECE_POSITION_KEYS: [[u64; 64]; 12] = {
        // generate random keys in byte form
        let bytes: [u8; 8 * 64 * 12] = const_random!([u8; 6144]);

        // memory for keys
        let mut keys: [[u64; 64]; 12] = [[0; 64]; 12];

        // turn bytes to u64s
        let mut p = 0;
        while p < 12 {
            let mut i = 0;
            while i < 64 {
                let mut current_bytes = [0; 8];
                let mut j = 0;
                while j < 8 {
                    current_bytes[j] = bytes[64*8*p + 8*i + j];
                    j += 1;
                }
                keys[p][i] = Self::u64_from_bytes(current_bytes);
                i += 1;
            }
            p += 1;
        }

        // return
        keys
    };
    pub const EN_PASSANT_KEYS: [u64; 64] = {
        // generate random keys in byte form
        let bytes: [u8; 512] = const_random!([u8; 512]);

        // memory for keys
        let mut keys: [u64; 64] = [0; 64];

        // turn bytes to u64s
        let mut i = 0;
        while i < 64 {
            let mut current_bytes = [0; 8];
            let mut j = 0;
            while j < 8 {
                current_bytes[j] = bytes[8*i + j];
                j += 1;
            }
            keys[i] = Self::u64_from_bytes(current_bytes);
            i += 1;
        }

        // return
        keys
    };
    const MOVE_COUNTER_KEYS: [u64; 150] = {
        // generate random keys in byte form
        let bytes: [u8; 8*150] = const_random!([u8; 1200]);

        // memory for keys
        let mut keys: [u64; 150] = [0; 150];

        // turn bytes to u64s
        let mut i = 0;
        while i < 150 {
            let mut current_bytes = [0; 8];
            let mut j = 0;
            while j < 8 {
                current_bytes[j] = bytes[8*i + j];
                j += 1;
            }
            keys[i] = Self::u64_from_bytes(current_bytes);
            i += 1;
        }

        // return
        keys
    };

    pub fn visualize(self: &Self) {
        // print a representation of the hash into console
        print!("{:#x}", self.0);
    }

    #[inline(always)]
    pub fn hash_player(self: &mut Self) {
        // hash player in/out
        self.0 ^= Self::PLAYER_KEY;
    }

    #[inline(always)]
    pub fn hash_piece(self: &mut Self, piece: Piece, square: Square) {
        // hash piece-position key in/out
        self.0 ^= Self::PIECE_POSITION_KEYS[piece as usize][square as usize];
    }

    #[inline(always)]
    pub fn hash_white_short(self: &mut Self) {
        // hash in/out key for this castling right
        self.0 ^= Self::WHITE_SHORT_KEY;
    }

    #[inline(always)]
    pub fn hash_white_long(self: &mut Self) {
        // hash in/out key for this castling right
        self.0 ^= Self::WHITE_LONG_KEY;
    }

    #[inline(always)]
    pub fn hash_black_short(self: &mut Self) {
        // hash in/out key for this castling right
        self.0 ^= Self::BLACK_SHORT_KEY;
    }

    #[inline(always)]
    pub fn hash_black_long(self: &mut Self) {
        // hash in/out key for this castling right
        self.0 ^= Self::BLACK_LONG_KEY;
    }

    #[inline(always)]
    pub fn hash_en_passant(self: &mut Self, square: Square) {
        // hash in/out key for the given en passant square
        self.0 ^= Self::EN_PASSANT_KEYS[square as usize];
    }

    #[inline(always)]
    pub fn hash_move_count(self: &mut Self, move_count: usize) {
        // hash in/out key for 50/75 move count
        self.0 ^= Self::MOVE_COUNTER_KEYS[move_count];
    }

    pub fn empty() -> Self {
        // returns an empty hash
        return Self(0);
    }
}
