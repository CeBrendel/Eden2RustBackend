
/*
TODO:
    - flip(x) is some bit operation on the u8 repr?
    - implement indexing (nastyyyy -- needs to be done for every type, ie arrays of different lengths)
*/

use num_derive::FromPrimitive;

// adds/removes asserts at compile time
const DO_ASSERTS: bool = false;

#[derive(Clone, Copy, PartialEq, FromPrimitive, Debug)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl Square {
    // for each square give the vertically flipped square
    const FLIPPED_SQUARES: [Self; 64] = [
        Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
    ];

    /*// for each square give the square of one rank higher
    const ADVANCE_FOR_WHITE: [Self; 64] = [
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
        Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8,
        Self::A1, Self::A1, Self::A1, Self::A1, Self::A1, Self::A1, Self::A1, Self::A1  // last row is a dummy
    ];*/

    /*// for each square give the square of one rank lower
    const ADVANCE_FOR_BLACK: [Self; 64] = [
        Self::A1, Self::A1, Self::A1, Self::A1, Self::A1, Self::A1, Self::A1, Self::A1, // first row is a dummy
        Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7
    ];*/

    #[inline(always)]
    pub const fn from_repr(repr: u8) -> Self {
        // turn the u8 representation of a square into the square
        if DO_ASSERTS {assert!((repr < 64));}
        return unsafe {std::mem::transmute(repr)}
    }

    #[inline(always)]
    pub const fn from_file_and_rank(file: u8, rank: u8) -> Self {
        // given a file and rank, produce the corresponding square
        if DO_ASSERTS {assert!((file + 8*rank) < 64);}
        return Self::from_repr(file + 8*rank);
    }

    #[inline(always)]
    pub const fn to_file_and_rank(self) -> (u8, u8) {
        // return file and rank corresponding to square
        return (self as u8 % 8, self as u8 / 8);
    }

    #[inline(always)]
    pub fn flip(self: Self) -> Self {
        // flip square vertically
        return Self::FLIPPED_SQUARES[self as usize];
    }

    #[inline(always)]
    pub fn advance_square(self, for_white: bool) -> Self {
        // advance the given square for either white (one rank higher) or black (one rank lower)
        return if for_white {
            if DO_ASSERTS {assert!((self as usize) < 56);}
            Self::from_repr(self as u8 + 8)
        } else {
            if DO_ASSERTS {assert!((self as usize) >= 8);}
            Self::from_repr(self as u8 - 8)
        }
    }

    pub const FILE_CHARS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    pub const RANK_CHARS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
    pub fn visualize(self: &Self) {
        let (file, rank) = self.to_file_and_rank();
        print!("{}{}", Self::FILE_CHARS[file as usize], Self::RANK_CHARS[rank as usize]);
    }

    pub fn to_string(self: &Self) -> String {
        let (file, rank) = self.to_file_and_rank();
        return format!(
            "{}{}", Self::FILE_CHARS[file as usize], Self::RANK_CHARS[rank as usize]
        );
    }
    pub fn from_algebraic(square: &str) -> Self {
        // parse &str
        let mut chars = square.chars();
        let file_char = chars.next().expect("Invalid square!");
        let rank_char = chars.next().expect("Invalid square!");

        // parse file and rank
        let file = match file_char {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            _ => {panic!("Invalid file!");}
        };
        let rank = match rank_char {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            _ => {panic!("Invalid rank!");}
        };

        // return square
        Square::from_file_and_rank(file, rank)
    }
}