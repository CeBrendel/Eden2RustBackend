
/*
TODO:
    - flatten arrays and make an inline function that allows easier indexing
    - transpose PATH_WITHOUT_END to make access easier for get_pinmasks
*/

pub mod byte_magic;

use bitboards::Bitboard;
use crate::byte_magic::{include_lookup_1d, include_lookup_2d};

pub static X_PEXT_MASK: [Bitboard; 64] = include_lookup_1d!("x_pext_masks.bin", 64);
pub static PLUS_PEXT_MASK: [Bitboard; 64] = include_lookup_1d!("plus_pext_masks.bin", 64);
pub static KNIGHT_MASK: [Bitboard; 64] = include_lookup_1d!("knight_masks.bin", 64);
pub static BISHOP_MASK: [[Bitboard; 512]; 64] = include_lookup_2d!("bishop_masks.bin", 512, 64);
pub static ROOK_MASK: [[Bitboard; 4096]; 64] = include_lookup_2d!("rook_masks.bin", 4096, 64);
pub static KING_MASK: [Bitboard; 64] = include_lookup_1d!("king_masks.bin", 64);
pub static PATH_WITHOUT_END: [[Bitboard; 64]; 64] = include_lookup_2d!("path_without_end_masks.bin", 64, 64);
