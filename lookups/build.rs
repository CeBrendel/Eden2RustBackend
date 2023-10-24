
use bitboards::{
    squares::Square,
    Bitboard,
    bitloop
};

use bytemuck::{bytes_of, allocation::zeroed_box};
use std::{
    path::PathBuf,
    env::var_os,
    fs::write
};

static ANTI_FRAMES: [Bitboard; 64] = {
    // for each square the frame (if the square is on the edge, excluding the edge) for that square

    let mut masks: [Bitboard; 64] = [Bitboard(0); 64];

    let inner_rank: u64 = 0b0111_1110;
    let inner_file: u64 = 0x0001010101010100;

    let mut square: u8 = 0;
    while square < 64 {

        let (file, rank) = Square::from_repr(square).to_file_and_rank();

        let mut mask = !0xFF818181818181FF;  // !(outer edge)

        if file == 0 {mask |= inner_file}  // on left edge?
        if file == 7 {mask |= inner_file << 7}  // on right edge?
        if rank == 0 {mask |= inner_rank}  // on bottom edge?
        if rank == 7 {mask |= inner_rank << 7*8}  // on top edge?

        // on corner?
        if (file==7 && rank==7)||(file==0 && rank==7)||(file==0 && rank==0)||(file==7 && rank==0) {
            mask |= 1 << square
        }

        masks[square as usize] = Bitboard(mask);

        square += 1;
    }

    masks
};

static X_MASK: [Bitboard; 64] = {
    // for each square the mask of shape "x" through that square

    let mut masks: [Bitboard; 64] = [Bitboard(0); 64];

    let mut square: u8 = 0;
    while square < 64 {

        let mut mask: u64 = 0;

        let (file, rank) = Square::from_repr(square).to_file_and_rank();

        // up-right ray
        let mut d_file: u8 = 0;
        let mut d_rank: u8 = 0;
        while file + d_file < 8 && rank + d_rank < 8 {
            mask |= (1 << square) << (d_file + 8*d_rank);
            d_file += 1;
            d_rank += 1
        }

        // up-left ray
        let mut d_file: u8 = 0;
        let mut d_rank: u8 = 0;
        while file >= d_file && rank + d_rank < 8 {
            mask |= (1 << square) << (8*d_rank - d_file);
            d_file += 1;
            d_rank += 1
        }

        // down-left ray
        let mut d_file: u8 = 0;
        let mut d_rank: u8 = 0;
        while file >= d_file && rank >= d_rank {
            mask |= (1 << square) >> (d_file + 8*d_rank);
            d_file += 1;
            d_rank += 1
        }

        // down-right ray
        let mut d_file: u8 = 0;
        let mut d_rank: u8 = 0;
        while file + d_file < 8 && rank >= d_rank {
            mask |= (1 << square) >> (8*d_rank - d_file);
            d_file += 1;
            d_rank += 1
        }

        masks[square as usize] = Bitboard(mask);

        square += 1;
    }

    masks
};

static PLUS_MASK: [Bitboard; 64] = {
    // for every square the mask of shape "+" through that square

    let mut plus_masks: [Bitboard; 64] = [Bitboard(0); 64];

    let first_rank: u64 = 0xFF;
    let first_file: u64 = 0x0101010101010101;

    // loop through all squares
    let mut sq: u8 = 0;
    while sq < 64 {

        let (file, rank) = Square::from_repr(sq).to_file_and_rank();

        plus_masks[sq as usize] = Bitboard(
            (first_rank << 8 * rank) | (first_file << file)
        );

        sq += 1;
    }

    plus_masks
};

static X_PEXT_MASK: [Bitboard; 64] = {
    // for each square the corresponding PEXT mask through that square

    let mut masks: [Bitboard; 64] = [Bitboard(0); 64];

    let mut square: usize = 0;
    while square < 64 {

        masks[square] = X_MASK[square].and(ANTI_FRAMES[square]).and(Bitboard(!(1 << square)));

        square += 1;
    }

    masks
};

static PLUS_PEXT_MASK: [Bitboard; 64] = {
    // for each square the corresponding PEXT mask through that square

    let mut masks: [Bitboard; 64] = [Bitboard(0); 64];

    let mut square: usize = 0;
    while square < 64 {

        masks[square] = PLUS_MASK[square].and(ANTI_FRAMES[square]).and(Bitboard(!(1 << square)));

        square += 1;
    }

    masks
};

fn get_knight_masks() -> Box<[Bitboard; 64]> {
    // for every square the mask of pseudo legal knight moves from that square

    let mut masks: Box<[Bitboard; 64]> = zeroed_box();

    // loop through all squares
    let mut square: u8 = 0;
    while square < 64 {

        let mut mask: u64 = 0u64;

        let (file, rank) = Square::from_repr(square).to_file_and_rank();

        // add one move at a time, paying attention to whether it leaves the board
        if (file >= 1) & (rank >= 2) {mask |= 1 << (square - 16 - 1)};
        if (file <= 6) & (rank >= 2) {mask |= 1 << (square - 16 + 1)};
        if (file >= 2) & (rank >= 1) {mask |= 1 << (square - 8 - 2)};
        if (file <= 5) & (rank >= 1) {mask |= 1 << (square - 8 + 2)};
        if (file >= 2) & (rank <= 6) {mask |= 1 << (square + 8 - 2)};
        if (file <= 5) & (rank <= 6) {mask |= 1 << (square + 8 + 2)};
        if (file >= 1) & (rank <= 5) {mask |= 1 << (square + 16 - 1)};
        if (file <= 6) & (rank <= 5) {mask |= 1 << (square + 16 + 1)};

        masks[square as usize] = Bitboard(mask);

        square += 1;
    }

    masks
}

fn get_bishop_masks() -> Box<[[Bitboard; 512]; 64]> {
    /*
    for each square and each possible occupancy of the "x" mask in that square (ignoring edges and
    centers, so at most 9 possible bits = 2^9 = 512 different occupancies) give the mask of pseudo
    legal moves
    */

    let mut masks: Box<[[Bitboard; 512]; 64]> = zeroed_box();

    let mut square: u8 = 0;
    while square < 64 {
        let mut pext_occupancy: u16 = 0;
        while pext_occupancy < 512 {

            // unpack occupancy
            let pdep_mask: Bitboard = X_PEXT_MASK[square as usize];
            let occupancy = Bitboard::const_pdep(pext_occupancy as u64, pdep_mask);

            let mut mask: u64 = 0;

            let (file, rank) = Square::from_repr(square).to_file_and_rank();

            // up-right ray
            let mut d_file: u8 = 1;
            let mut d_rank: u8 = 1;
            while file + d_file < 8 && rank + d_rank < 8 {
                let d_square = d_file + 8*d_rank;
                mask |= (1 << square) << d_square;
                if occupancy.has_entry_at(Square::from_repr(square + d_square)) {
                    break;
                }
                d_file += 1;
                d_rank += 1;
            }

            // up-left ray
            let mut d_file: u8 = 1;
            let mut d_rank: u8 = 1;
            while file >= d_file && rank + d_rank < 8 {
                let d_square = 8*d_rank - d_file;
                mask |= (1 << square) << d_square;
                if occupancy.has_entry_at(Square::from_repr(square + d_square)) {
                    break;
                }
                d_file += 1;
                d_rank += 1;
            }

            // down-left ray
            let mut d_file: u8 = 1;
            let mut d_rank: u8 = 1;
            while file >= d_file && rank >= d_rank {
                let d_square = d_file + 8*d_rank;
                mask |= (1 << square) >> d_square;
                if occupancy.has_entry_at(Square::from_repr(square - d_square)) {
                    break;
                }
                d_file += 1;
                d_rank += 1;
            }

            // down-right ray
            let mut d_file: u8 = 1;
            let mut d_rank: u8 = 1;
            while file + d_file < 8 && rank >= d_rank {
                let d_square = 8*d_rank - d_file;
                mask |= (1 << square) >> d_square;
                if occupancy.has_entry_at(Square::from_repr(square - d_square)) {
                    break;
                }
                d_file += 1;
                d_rank += 1;
            }

            masks[square as usize][pext_occupancy as usize] = Bitboard(mask);

            pext_occupancy += 1;
        }
        square += 1;
    }

    masks
}

fn get_rook_masks() -> Box<[[Bitboard; 4096]; 64]> {
    /*
    for each square and each possible occupancy of the "+" mask in that square (ignoring edges and
    centers, so 11 possible bits = 2^11 = 4096 different occupancies) give the mask of pseudo
    legal moves
     */

    let mut masks: Box<[[Bitboard; 4096]; 64]> = zeroed_box();

    let mut square: u8 = 0;
    while square < 64 {
        let mut pext_occupancy: u16 = 0;
        while pext_occupancy < 4096 {

            // unpack occupancy
            let pdep_mask: Bitboard = PLUS_PEXT_MASK[square as usize];
            let occupancy = Bitboard::const_pdep(pext_occupancy as u64, pdep_mask);


            let mut mask: u64 = 0;

            let (file, rank) = Square::from_repr(square).to_file_and_rank();

            // right ray
            let mut d_file: u8 = 1;
            while file + d_file < 8 {
                mask |= (1 << square) << d_file;
                if occupancy.has_entry_at(Square::from_repr(square + d_file)) {
                    break;
                }
                d_file += 1;
            }

            // up ray
            let mut d_rank: u8 = 1;
            while rank + d_rank < 8 {
                mask |= (1 << square) << 8*d_rank;
                if occupancy.has_entry_at(Square::from_repr(square + 8*d_rank)) {
                    break;
                }
                d_rank += 1;
            }

            // left ray
            let mut d_file: u8 = 1;
            while file >= d_file {
                mask |= (1 << square) >> d_file;
                if occupancy.has_entry_at(Square::from_repr(square - d_file)) {
                    break;
                }
                d_file += 1;
            }

            // down ray
            let mut d_rank: u8 = 1;
            while rank >= d_rank {
                mask |= (1 << square) >> 8*d_rank;
                if occupancy.has_entry_at(Square::from_repr(square - 8*d_rank)) {
                    break;
                }
                d_rank += 1;
            }

            masks[square as usize][pext_occupancy as usize] = Bitboard(mask);

            pext_occupancy += 1;
        }
        square += 1;
    }

    masks
}

fn get_king_masks() -> Box<[Bitboard; 64]> {
    // for each square get the pseudo legal moves of a king at that square

    let mut masks: Box<[Bitboard; 64]> = zeroed_box();

    let mut square: u8 = 0;
    while square < 64 {

        let (file, rank) = Square::from_repr(square).to_file_and_rank();

        let mut mask: u64 = 0;

        if file < 7 {mask |= (1 << square) << 1}
        if 0 < file {mask |= (1 << square) >> 1}
        if rank < 7 {mask |= (1 << square) << 8}
        if 0 < rank {mask |= (1 << square) >> 8}
        if file < 7 && rank < 7 {mask |= (1 << square) << (8 + 1)}
        if file > 0 && rank < 7 {mask |= (1 << square) << (8 - 1)}
        if file > 0 && rank > 0 {mask |= (1 << square) >> (8 + 1)}
        if file < 7 && rank > 0 {mask |= (1 << square) >> (8 - 1)}

        masks[square as usize] = Bitboard(mask);

        square += 1;
    }

    masks
}

fn get_path_without_end_masks() -> Box<[[Bitboard; 64]; 64]> {
    /*
    For a given from- and to-square the path (either x- or plus-path) excluding the to-square
    */

    let mut masks: Box<[[Bitboard; 64]; 64]> = zeroed_box();

    let mut to_square: u8 = 0;
    while to_square < 64 {

        let (to_file, to_rank) = Square::from_repr(to_square).to_file_and_rank();

        // handle x-paths
        let from_squares = X_MASK[to_square as usize].and(Bitboard(!(1 << to_square)));
        bitloop!(
            from_squares, from_square => {

                let (from_file, from_rank) = Square::from_repr(from_square).to_file_and_rank();

                let mut mask: u64 = 0;

                if from_file < to_file && from_rank < to_rank {
                    // up-right
                    let mut d_file: u8 = 0;
                    let mut d_rank: u8 = 0;
                    while from_file + d_file < to_file && from_rank + d_rank < to_rank {
                        mask |= (1 << from_square) << (d_file + 8*d_rank);
                        d_file += 1;
                        d_rank += 1;
                    }
                } else if from_file > to_file && from_rank < to_rank {
                    // up-left
                    let mut d_file: u8 = 0;
                    let mut d_rank: u8 = 0;
                    while from_file - d_file > to_file && from_rank + d_rank < to_rank {
                        mask |= (1 << from_square) << (8*d_rank - d_file);
                        d_file += 1;
                        d_rank += 1;
                    }
                } else if from_file > to_file && from_rank > to_rank {
                    // down-left
                    let mut d_file: u8 = 0;
                    let mut d_rank: u8 = 0;
                    while from_file - d_file > to_file && from_rank - d_rank > to_rank {
                        mask |= (1 << from_square) >> (d_file + 8*d_rank);
                        d_file += 1;
                        d_rank += 1;
                    }
                } else {
                    // down-right
                    let mut d_file: u8 = 0;
                    let mut d_rank: u8 = 0;
                    while from_file + d_file < to_file && from_rank - d_rank > to_rank {
                        mask |= (1 << from_square) >> (8*d_rank - d_file);
                        d_file += 1;
                        d_rank += 1;
                    }
                }

                masks[from_square as usize][to_square as usize] = Bitboard(mask);

            }
        );

        // handle plus-paths
        let from_squares = PLUS_MASK[to_square as usize].and(Bitboard(!(1 << to_square)));
        bitloop!(
            from_squares, from_square => {

                let (from_file, from_rank) = Square::from_repr(from_square).to_file_and_rank();

                let mut mask: u64 = 0;

                if from_file < to_file {
                    // right
                    let mut d_file: u8 = 0;
                    while from_file + d_file < to_file {
                        mask |= (1 << from_square) << d_file;
                        d_file += 1;
                    }
                } else if from_rank < to_rank {
                    // up
                    let mut d_rank: u8 = 0;
                    while from_rank + d_rank < to_rank {
                        mask |= (1 << from_square) << (8*d_rank);
                        d_rank += 1;
                    }
                } else if from_file > to_file {
                    // left
                    let mut d_file: u8 = 0;
                    while from_file - d_file > to_file {
                        mask |= (1 << from_square) >> d_file;
                        d_file += 1;
                    }
                } else {
                    // down
                    let mut d_rank: u8 = 0;
                    while from_rank - d_rank > to_rank {
                        mask |= (1 << from_square) >> (8*d_rank);
                        d_rank += 1;
                    }
                }

                masks[from_square as usize][to_square as usize] = Bitboard(mask);

            }
        );

        to_square += 1;
    }

    masks
}

fn write_from_suffix(data: &[u8], suffix: &str) {
    let out_dir = PathBuf::from(var_os("OUT_DIR").unwrap());
    write(out_dir.join(suffix), data).expect("Writing failed!");
}

fn write_x_pext_masks() {
    let mut container = zeroed_box::<[Bitboard; 64]>();
    *container = X_PEXT_MASK;
    let bytes = bytes_of(&*container);
    write_from_suffix(bytes, "x_pext_masks.bin");
}

fn write_plus_pext_masks() {
    // write into container, cast to bytes and write to file
    let mut container = zeroed_box::<[Bitboard; 64]>();
    *container = PLUS_PEXT_MASK;
    let bytes = bytes_of(&*container);
    write_from_suffix(bytes, "plus_pext_masks.bin");
}

fn write_knight_masks() {
    let container = get_knight_masks();
    let bytes = bytes_of(&*container);
    write_from_suffix(bytes, "knight_masks.bin");
}

fn write_bishop_masks() {
    let container = get_bishop_masks();
    let bytes = bytes_of(&*container);
    write_from_suffix(bytes, "bishop_masks.bin");
}

fn write_rook_masks() {
    let container = get_rook_masks();
    let bytes = bytes_of(&*container);
    write_from_suffix(bytes, "rook_masks.bin");
}

fn write_king_masks() {
    let container = get_king_masks();
    let bytes = bytes_of(&*container);
    write_from_suffix(bytes, "king_masks.bin");
}

fn write_path_without_end_masks() {
    let container = get_path_without_end_masks();
    let bytes = bytes_of(&*container);
    write_from_suffix(bytes, "path_without_end_masks.bin");
}


fn main() {
    write_x_pext_masks();
    write_plus_pext_masks();
    write_knight_masks();
    write_bishop_masks();
    write_rook_masks();
    write_king_masks();
    write_path_without_end_masks();

    // prevent unnecessary recompilation of the lookup tables
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../bitboards/");
}